use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::{
    associations::HasTable, insert_into, Connection, Insertable, PgConnection, RunQueryDsl,
};
use dotenvy::dotenv;
use ethers::{
    abi::{Abi, Address, ParamType},
    contract::abigen,
    core::{abi::decode, types::Filter},
    providers::{Http, Middleware, Provider},
    types::{Log, H256, U256},
    utils::keccak256,
};

use eyre::Result;

pub mod models;
pub mod schema;

use crate::{models::SyncEvent, schema::sync_events::dsl::sync_events};
use std::{env, str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = &mut PgConnection::establish(&database_url)?;

    let rpc_http_url = env::var("RPC_HTTP_URL").expect("Missing RPC_HTTP_URL env var");

    let sync_event_signature_hash = event_signature_hash("Sync(uint112,uint112)");

    let provider = Provider::<Http>::try_from(rpc_http_url)?;

    let client = Arc::new(provider);

    let events_signatures_hashes = vec![sync_event_signature_hash];

    let step = 3000;

    let filter = Filter::new().topic0(events_signatures_hashes);

    for from_block in (30170377..31340138).step_by(step) {
        let to_block = from_block + step;

        let filter = filter.clone().from_block(from_block).to_block(to_block);

        let logs = client.get_logs(&filter).await?;

        println!(
            "Pulled {} new logs from block {from_block} to block {to_block}",
            logs.iter().len()
        );

        let new_sync_events = process_logs(logs)?;

        let _ = insert_into(sync_events::table())
            .values(new_sync_events)
            .execute(connection);

        println!("Inserted events to the database");
    }

    Ok(())
}

fn event_signature_hash(event_signature: &str) -> H256 {
    H256::from(keccak256(event_signature.as_bytes()))
}

fn process_logs(logs: Vec<Log>) -> Result<Vec<SyncEvent>> {
    let mut new_sync_events: Vec<SyncEvent> = vec![];

    for log in logs.iter() {
        let Log {
            block_number,
            log_index,
            transaction_hash,
            transaction_index,
            data,
            address,
            ..
        } = log;

        let types = [ParamType::Uint(112), ParamType::Uint(112)];

        let output = decode(&types, data)?;

        let reserves = output
            .into_iter()
            .map(|t| t.into_uint().expect("Invalid reserve uint"))
            .collect::<Vec<U256>>();

        let reserve_0 =
            BigDecimal::from_u128(reserves[0].as_u128()).expect("reserve0 BigDecimal from u128");
        let reserve_1 =
            BigDecimal::from_u128(reserves[1].as_u128()).expect("reserve1 BigDecimal from u128");

        let new_sync_event = SyncEvent {
            address: format!("{:?}", address),
            transaction_hash: format!("{:?}", transaction_hash.expect("None transaction_hash")),
            block_number: block_number.expect("None block_number").as_u64() as i64,
            transaction_index: transaction_index.expect("None transaction_index").as_u32() as i32,
            log_index: log_index.expect("None log_index").as_u32() as i32,
            reserve_0,
            reserve_1,
        };

        new_sync_events.push(new_sync_event);
    }

    Ok(new_sync_events)
}
