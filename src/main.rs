use dotenvy::dotenv;
use ethers::{
    abi::{Abi, Address, ParamType},
    contract::abigen,
    core::{abi::decode, types::Filter},
    providers::{Http, Middleware, Provider},
    types::{H256, U256},
    utils::keccak256,
};
use eyre::Result;
use std::{env, str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let rpc_http_url = env::var("RPC_HTTP_URL").expect("Missing RPC_HTTP_URL env var");

    let sync_event_signature_hash = event_signature_hash("Sync(uint112,uint112)");

    let provider = Provider::<Http>::try_from(rpc_http_url)?;
    let client = Arc::new(provider);

    let events_signatures_hashes = vec![sync_event_signature_hash];

    let filter = Filter::new()
        .topic0(events_signatures_hashes)
        .from_block(31307816)
        .to_block(31307950);
    let logs = client.get_logs(&filter).await?;

    println!("{}", logs.iter().len());

    for log in logs.iter() {
        let block_number = log.block_number.expect("unknown block_number");

        let types = [ParamType::Uint(112), ParamType::Uint(112)];

        let output = decode(&types, &log.data)?;

        let reserves = output
            .into_iter()
            .map(|t| t.into_uint().expect("Invalid reserve uint"))
            .collect::<Vec<U256>>();

        let reserve0 = reserves[0];
        let reserve1 = reserves[1];

        println!("{:?}  {:?}", reserve0, reserve1);
    }
    Ok(())
}

fn event_signature_hash(event_signature: &str) -> H256 {
    H256::from(keccak256(event_signature.as_bytes()))
}
