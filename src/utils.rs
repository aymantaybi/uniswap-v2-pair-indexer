use std::str::FromStr;

use bigdecimal::BigDecimal;
use ethers::abi::Token;

pub fn token_to_big_decimal(token: Token) -> BigDecimal {
    let amount = token.into_uint().expect("Token to uint");
    BigDecimal::from_str(&amount.to_string()).expect("BigDecimal from string")
}
