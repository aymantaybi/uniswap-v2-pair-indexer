use bigdecimal::BigDecimal;
use diesel::prelude::*;
use ethers::{
    abi::{decode, ParamType},
    types::Log,
};
use eyre::Result;

use crate::helpers::extract_event_base_details;
use crate::utils::token_to_big_decimal;

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::sync_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(block_number, transaction_index, log_index))]
pub struct SyncEvent {
    pub address: String,
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_hash: String,
    pub reserve_0: BigDecimal,
    pub reserve_1: BigDecimal,
}

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::mint_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(block_number, transaction_index, log_index))]
pub struct MintEvent {
    pub address: String,
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_hash: String,
    pub amount_0: BigDecimal,
    pub amount_1: BigDecimal,
}

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::burn_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(block_number, transaction_index, log_index))]
pub struct BurnEvent {
    pub address: String,
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_hash: String,
    pub amount_0: BigDecimal,
    pub amount_1: BigDecimal,
}

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::swap_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(block_number, transaction_index, log_index))]
pub struct SwapEvent {
    pub address: String,
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_hash: String,
    pub amount_0_in: BigDecimal,
    pub amount_1_in: BigDecimal,
    pub amount_0_out: BigDecimal,
    pub amount_1_out: BigDecimal,
}

impl TryFrom<Log> for SyncEvent {
    type Error = &'static str;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let (address, transaction_hash, block_number, transaction_index, log_index) =
            extract_event_base_details(&log);

        let types = [ParamType::Uint(112), ParamType::Uint(112)];

        let output = decode(&types, &log.data).map_err(|err| "Cannot decode log data")?;

        let reserves = output
            .into_iter()
            .map(token_to_big_decimal)
            .collect::<Vec<BigDecimal>>();

        let mut iter = reserves.into_iter();

        let reserve_0 = iter.next().expect("Missing reserve_0");
        let reserve_1 = iter.next().expect("Missing reserve_1");

        Ok(SyncEvent {
            address,
            transaction_hash,
            block_number,
            transaction_index,
            log_index,
            reserve_0,
            reserve_1,
        })
    }
}

impl TryFrom<Log> for MintEvent {
    type Error = &'static str;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let (address, transaction_hash, block_number, transaction_index, log_index) =
            extract_event_base_details(&log);

        let types = [ParamType::Uint(256), ParamType::Uint(256)];

        let output = decode(&types, &log.data).map_err(|err| "Cannot decode log data")?;

        let amounts = output
            .into_iter()
            .map(token_to_big_decimal)
            .collect::<Vec<BigDecimal>>();

        let mut iter = amounts.into_iter();

        let amount_0 = iter.next().expect("Missing amount_0");
        let amount_1 = iter.next().expect("Missing amount_1");

        Ok(MintEvent {
            address,
            transaction_hash,
            block_number,
            transaction_index,
            log_index,
            amount_0,
            amount_1,
        })
    }
}

impl TryFrom<Log> for BurnEvent {
    type Error = &'static str;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let (address, transaction_hash, block_number, transaction_index, log_index) =
            extract_event_base_details(&log);

        let types = [ParamType::Uint(256), ParamType::Uint(256)];

        let output = decode(&types, &log.data).map_err(|err| "Cannot decode log data")?;

        let amounts = output
            .into_iter()
            .map(token_to_big_decimal)
            .collect::<Vec<BigDecimal>>();

        let mut iter = amounts.into_iter();

        let amount_0 = iter.next().expect("Missing amount_0");
        let amount_1 = iter.next().expect("Missing amount_1");

        Ok(BurnEvent {
            address,
            transaction_hash,
            block_number,
            transaction_index,
            log_index,
            amount_0,
            amount_1,
        })
    }
}

impl TryFrom<Log> for SwapEvent {
    type Error = &'static str;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let (address, transaction_hash, block_number, transaction_index, log_index) =
            extract_event_base_details(&log);

        let types = [
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Uint(256),
        ];

        let output = decode(&types, &log.data).map_err(|err| "Cannot decode log data")?;

        let amounts = output
            .into_iter()
            .map(token_to_big_decimal)
            .collect::<Vec<BigDecimal>>();

        let mut iter = amounts.into_iter();

        let amount_0_in = iter.next().expect("Missing amount_0_in");
        let amount_1_in = iter.next().expect("Missing amount_1_in");
        let amount_0_out = iter.next().expect("Missing amount_0_out");
        let amount_1_out = iter.next().expect("Missing amount_1_out");

        Ok(SwapEvent {
            address,
            transaction_hash,
            block_number,
            transaction_index,
            log_index,
            amount_0_in,
            amount_1_in,
            amount_0_out,
            amount_1_out,
        })
    }
}
