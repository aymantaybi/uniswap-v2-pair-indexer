use diesel::{insert_into, Connection, PgConnection, RunQueryDsl};
use dotenvy::dotenv;
use ethers::{
    core::types::Filter,
    providers::{Http, Middleware, Provider},
    types::Log,
};

use eyre::Result;
use helpers::event_signature_hash;
use log::{debug, error, info, warn};
use std::{env, sync::Arc};

pub mod helpers;
pub mod models;
pub mod schema;
pub mod utils;

use crate::{
    models::{BurnEvent, MintEvent, SwapEvent, SyncEvent},
    schema::{burn_events, mint_events, swap_events, sync_events},
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = &mut PgConnection::establish(&database_url)?;

    let rpc_http_url = env::var("RPC_HTTP_URL").expect("Missing RPC_HTTP_URL env var");

    let provider = Provider::<Http>::try_from(rpc_http_url)?;

    let client = Arc::new(provider);

    let sync_event_signature_hash = event_signature_hash("Sync(uint112,uint112)");

    let mint_event_signature_hash = event_signature_hash("Mint(address,uint256,uint256)");

    let burn_event_signature_hash = event_signature_hash("Burn(address,uint256,uint256,address)");

    let swap_event_signature_hash =
        event_signature_hash("Swap(address,uint256,uint256,uint256,uint256,address)");

    let events_signatures_hashes = vec![
        sync_event_signature_hash,
        mint_event_signature_hash,
        burn_event_signature_hash,
        swap_event_signature_hash,
    ];

    let step = 3000;

    let filter = Filter::new().topic0(events_signatures_hashes);

    for from_block in (30170377..31340138).step_by(step) {
        let to_block = from_block + step;

        let filter = filter.clone().from_block(from_block).to_block(to_block);

        let logs = client.get_logs(&filter).await?;

        info!(
            "Pulled {} new logs from block {from_block} to block {to_block}",
            logs.iter().len()
        );

        let (new_sync_events, new_mint_events, new_burn_events, new_swap_events) =
            process_logs(logs)?;

        info!(
            "Inserted {} Sync event to the database",
            new_sync_events.iter().len()
        );
        info!(
            "Inserted {} Mint event to the database",
            new_mint_events.iter().len()
        );
        info!(
            "Inserted {} Burn event to the database",
            new_burn_events.iter().len()
        );
        info!(
            "Inserted {} Swap event to the database",
            new_swap_events.iter().len()
        );

        let _ = insert_into(sync_events::table)
            .values(new_sync_events)
            .execute(connection);

        let _ = insert_into(mint_events::table)
            .values(new_mint_events)
            .execute(connection);

        let _ = insert_into(burn_events::table)
            .values(new_burn_events)
            .execute(connection);

        let _ = insert_into(swap_events::table)
            .values(new_swap_events)
            .execute(connection);
    }

    Ok(())
}

fn process_logs(
    logs: Vec<Log>,
) -> Result<(
    Vec<SyncEvent>,
    Vec<MintEvent>,
    Vec<BurnEvent>,
    Vec<SwapEvent>,
)> {
    let mut new_sync_events: Vec<SyncEvent> = vec![];
    let mut new_mint_events: Vec<MintEvent> = vec![];
    let mut new_burn_events: Vec<BurnEvent> = vec![];
    let mut new_swap_events: Vec<SwapEvent> = vec![];

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
                new_mint_events.push(new_mint_event);
            }
            "0xdccd412f0b1252819cb1fd330b93224ca42612892bb3f4f789976e6d81936496" => {
                // Burn
                let new_burn_event =
                    BurnEvent::try_from(log).expect("Cannot convert Log to BurnEvent");
                new_burn_events.push(new_burn_event);
            }
            "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822" => {
                // Swap
                let new_swap_event =
                    SwapEvent::try_from(log).expect("Cannot convert Log to SwapEvent");
                new_swap_events.push(new_swap_event);
            }
            &_ => todo!(),
        };
    }

    Ok((
        new_sync_events,
        new_mint_events,
        new_burn_events,
        new_swap_events,
    ))
}
