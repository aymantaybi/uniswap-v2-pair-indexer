// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use ethers::{
    abi::{decode, ParamType},
    types::{Log, U256},
};
use eyre::Result;

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

impl TryFrom<Log> for SyncEvent {
    type Error = &'static str;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let Log {
            block_number,
            log_index,
            transaction_hash,
            transaction_index,
            data,
            address,
            ..
        } = log;

        let types = [ParamType::Uint(112), ParamType::Uint(112)];

        let output = decode(&types, &data).map_err(|err| "Cannot decode log data")?;

        let reserves = output
            .into_iter()
            .map(|t| t.into_uint().expect("Invalid reserve uint"))
            .collect::<Vec<U256>>();

        let reserve_0 =
            BigDecimal::from_u128(reserves[0].as_u128()).expect("reserve0 BigDecimal from u128");
        let reserve_1 =
            BigDecimal::from_u128(reserves[1].as_u128()).expect("reserve1 BigDecimal from u128");

        Ok(SyncEvent {
            address: format!("{:?}", address),
            transaction_hash: format!("{:?}", transaction_hash.expect("None transaction_hash")),
            block_number: block_number.expect("None block_number").as_u64() as i64,
            transaction_index: transaction_index.expect("None transaction_index").as_u32() as i32,
            log_index: log_index.expect("None log_index").as_u32() as i32,
            reserve_0,
            reserve_1,
        })
    }
}

impl TryFrom<Log> for MintEvent {
    type Error = &'static str;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let Log {
            block_number,
            log_index,
            transaction_hash,
            transaction_index,
            data,
            address,
            ..
        } = log;

        let types = [ParamType::Uint(256), ParamType::Uint(256)];

        let output = decode(&types, &data).map_err(|err| "Cannot decode log data")?;

        let amounts = output
            .into_iter()
            .map(|t| t.into_uint().expect("Invalid reserve uint"))
            .collect::<Vec<U256>>();

        let amount_0 =
            BigDecimal::from_str(&amounts[0].to_string()).expect("amount_0 BigDecimal from string");
        let amount_1 =
            BigDecimal::from_str(&amounts[1].to_string()).expect("amount_1 BigDecimal from string");

        Ok(MintEvent {
            address: format!("{:?}", address),
            transaction_hash: format!("{:?}", transaction_hash.expect("None transaction_hash")),
            block_number: block_number.expect("None block_number").as_u64() as i64,
            transaction_index: transaction_index.expect("None transaction_index").as_u32() as i32,
            log_index: log_index.expect("None log_index").as_u32() as i32,
            amount_0,
            amount_1,
        })
    }
}
