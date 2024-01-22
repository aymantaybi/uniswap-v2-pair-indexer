// @generated automatically by Diesel CLI.

diesel::table! {
    burn_events (block_number, transaction_index, log_index) {
        #[max_length = 42]
        address -> Varchar,
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
        amount_0 -> Numeric,
        amount_1 -> Numeric,
    }
}

diesel::table! {
    mint_events (block_number, transaction_index, log_index) {
        #[max_length = 42]
        address -> Varchar,
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
        amount_0 -> Numeric,
        amount_1 -> Numeric,
    }
}

diesel::table! {
    swap_events (block_number, transaction_index, log_index) {
        #[max_length = 42]
        address -> Varchar,
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
        amount_0_in -> Numeric,
        amount_1_in -> Numeric,
        amount_0_out -> Numeric,
        amount_1_out -> Numeric,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    burn_events,
    mint_events,
    swap_events,
    sync_events,
);
