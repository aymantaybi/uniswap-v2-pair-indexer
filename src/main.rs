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
use models::MintEvent;

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

    let provider = Provider::<Http>::try_from(rpc_http_url)?;

    let client = Arc::new(provider);

    let sync_event_signature_hash = event_signature_hash("Sync(uint112,uint112)");

    let mint_event_signature_hash = event_signature_hash("Mint(address,uint256,uint256)");

    let events_signatures_hashes = vec![sync_event_signature_hash, mint_event_signature_hash];

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

        let (new_sync_events, new_mint_events) = process_logs(logs)?;

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

fn process_logs(logs: Vec<Log>) -> Result<(Vec<SyncEvent>, Vec<MintEvent>)> {
    let mut new_sync_events: Vec<SyncEvent> = vec![];
    let mut new_mint_events: Vec<MintEvent> = vec![];

    for log in logs.into_iter() {
        let topic0 = format!("{:?}", log.topics[0]);
        match topic0.as_str() {
            "0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1" => {
                // Sync
                let new_sync_event =
                    SyncEvent::try_from(log).expect("Cannot convert Log to SyncEvent");
                new_sync_events.push(new_sync_event);
            }
            "0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f" => {
                // Mint
                let new_mint_event =
                    MintEvent::try_from(log).expect("Cannot convert Log to MintEvent");
                println!("{:?}", new_mint_event);
                new_mint_events.push(new_mint_event);
            }
            &_ => todo!(),
        };
    }

    Ok((new_sync_events, new_mint_events))
}
