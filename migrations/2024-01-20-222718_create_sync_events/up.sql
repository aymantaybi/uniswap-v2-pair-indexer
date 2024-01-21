CREATE TABLE sync_events (
    block_number BIGINT NOT NULL,
    transaction_index INTEGER NOT NULL,
    log_index INTEGER NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL,
    reserve_0 NUMERIC(34, 0) NOT NULL,
    reserve_1 NUMERIC(34, 0) NOT NULL,
    PRIMARY KEY (block_number, transaction_index, log_index),
    UNIQUE (block_number, transaction_index, log_index)
);