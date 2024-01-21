-- Your SQL goes here
ALTER TABLE
    sync_events
ADD
    COLUMN address VARCHAR(42) NOT NULL;