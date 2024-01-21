// @generated automatically by Diesel CLI.

diesel::table! {
    sync_events (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
        reserve_0 -> Numeric,
        reserve_1 -> Numeric,
        #[max_length = 42]
        address -> Varchar,
    }
}
