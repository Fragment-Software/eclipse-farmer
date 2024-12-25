use rand::{seq::SliceRandom, thread_rng};
use solana_sdk::pubkey::Pubkey;

use super::constants::{ETH_PUBKEY, SOL_PUBKEY, USDC_PUBKEY};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub mint: Pubkey,
    pub decimals: u8,
    pub symbol: &'static str,
    pub is_native: bool,
    pub coinlore_id: &'static str,
}

impl Token {
    pub const ETH: Token =
        Token { mint: ETH_PUBKEY, decimals: 9, symbol: "ETH", is_native: true, coinlore_id: "80" };

    pub const USDC: Token = Token {
        mint: USDC_PUBKEY,
        decimals: 6,
        symbol: "USDC",
        is_native: false,
        coinlore_id: "33285",
    };

    pub const SOL: Token = Token {
        mint: SOL_PUBKEY,
        decimals: 9,
        symbol: "SOL",
        is_native: false,
        coinlore_id: "48543",
    };

    pub fn from_coinlore_id(id: &str) -> Self {
        match id {
            "80" => Token::ETH,
            "48543" => Token::SOL,
            "33285" => Token::USDC,
            _ => unreachable!(),
        }
    }

    pub fn get_lifinity_paired_token(token_in: &Token) -> Token {
        match *token_in {
            Token::ETH | Token::SOL => Token::USDC,
            _ => [Token::ETH, Token::SOL]
                .choose(&mut thread_rng())
                .cloned()
                .expect("Token selection failed"),
        }
    }

    pub fn to_amount(token: &Self, ui_amount: f64) -> u64 {
        (ui_amount * 10f64.powi(token.decimals as i32)) as u64
    }

    pub fn to_ui_amount(token: &Self, amount: u64) -> f64 {
        amount as f64 / 10f64.powi(token.decimals as i32)
    }
}

impl TryFrom<Pubkey> for Token {
    type Error = eyre::Report;

    fn try_from(pubkey: Pubkey) -> Result<Self, Self::Error> {
        match pubkey {
            ETH_PUBKEY => Ok(Token::ETH),
            SOL_PUBKEY => Ok(Token::SOL),
            USDC_PUBKEY => Ok(Token::USDC),
            _ => Err(eyre::eyre!("Unknown token with pubkey: {pubkey}")),
        }
    }
}
