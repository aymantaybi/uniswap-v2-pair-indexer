-- Your SQL goes here
CREATE TABLE sync_events (
    id SERIAL PRIMARY KEY,
    transaction_hash VARCHAR(66) NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_index INTEGER NOT NULL,
    log_index INTEGER NOT NULL,
    reserve_0 NUMERIC NOT NULL,
    reserve_1 NUMERIC NOT NULL
);