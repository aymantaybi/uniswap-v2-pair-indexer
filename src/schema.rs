// @generated automatically by Diesel CLI.

diesel::table! {
    sync_events (id) {
        id -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        reserve_0 -> Numeric,
        reserve_1 -> Numeric,
    }
}
