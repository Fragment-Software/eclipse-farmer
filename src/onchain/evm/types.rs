use std::fmt::Display;

use alloy::primitives::{Address, U256};

#[allow(unused)]
#[derive(Clone)]
pub struct Token {
    pub contract_address: Option<Address>,
    pub decimals: u8,
    pub symbol: &'static str,
    pub is_erc20: bool,
}

impl Token {
    pub const ETH: Token =
        Token { contract_address: None, decimals: 18, symbol: "ETH", is_erc20: false };

    #[allow(unused)]
    pub fn to_wei(&self, amount: f64) -> U256 {
        U256::from(amount * 10f64.powi(self.decimals as i32))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.symbol)
    }
}
