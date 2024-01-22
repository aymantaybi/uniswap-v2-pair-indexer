-- Your SQL goes here
CREATE TABLE swap_events (
    address VARCHAR(42) NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_index INTEGER NOT NULL,
    log_index INTEGER NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL,
    amount_0_in NUMERIC(78, 0) NOT NULL,
    amount_1_in NUMERIC(78, 0) NOT NULL,
    amount_0_out NUMERIC(78, 0) NOT NULL,
    amount_1_out NUMERIC(78, 0) NOT NULL,
    PRIMARY KEY (block_number, transaction_index, log_index),
    UNIQUE (block_number, transaction_index, log_index)
);