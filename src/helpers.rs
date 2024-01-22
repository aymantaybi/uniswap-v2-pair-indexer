use ethers::types::Log;

pub fn extract_event_base_details(log: &Log) -> (String, String, i64, i32, i32) {
    let address = format!("{:?}", log.address);
    let transaction_hash = format!(
        "{:?}",
        log.transaction_hash.expect("Missing transaction_hash")
    );
    let block_number = log.block_number.expect("Missing block_number").as_u64() as i64;
    let transaction_index = log
        .transaction_index
        .expect("Missing transaction_index")
        .as_u32() as i32;
    let log_index = log.log_index.expect("Missing log_index").as_u32() as i32;

    (
        address,
        transaction_hash,
        block_number,
        transaction_index,
        log_index,
    )
}
