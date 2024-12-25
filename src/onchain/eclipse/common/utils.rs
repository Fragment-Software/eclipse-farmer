use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use reqwest::Proxy;
use solana_account_decoder_client_types::UiAccountData;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::pubkey::Pubkey;

use crate::{
    coinlore::api::get_tickers_usd_value, onchain::eclipse::common::typedefs::ParsedTokenAccount,
};

use super::{constants::TOKEN_2022_PROGRAM_ID, token::Token};

pub async fn get_token_with_largest_balance(
    provider: &RpcClient,
    owner: &Pubkey,
    proxy: Option<&Proxy>,
) -> eyre::Result<(Token, f64)> {
    let token_to_price_mapping = get_tickers_usd_value(proxy).await?;

    let (native_balance, spl_tokens) = tokio::try_join!(
        provider.get_balance(owner),
        provider.get_token_accounts_by_owner(
            owner,
            TokenAccountsFilter::ProgramId(TOKEN_2022_PROGRAM_ID)
        )
    )?;

    let mut token_to_usd_value_mapping = HashMap::new();
    let mut token_to_amount_mapping = HashMap::new();

    let native_amount = native_balance as f64 / 10f64.powi(Token::ETH.decimals as i32);
    let native_usd_value = native_amount * token_to_price_mapping.get(&Token::ETH).unwrap();

    token_to_usd_value_mapping.insert(Token::ETH, native_usd_value);
    token_to_amount_mapping.insert(Token::ETH, native_amount);

    for token in spl_tokens {
        if let UiAccountData::Json(parsed_account) = token.account.data {
            let account = serde_json::from_value::<ParsedTokenAccount>(parsed_account.parsed)?;
            let amount = account.info.token_amount.amount.parse::<f64>()?;
            let token_pubkey = Pubkey::from_str(&account.info.mint)?;

            let Ok(token) = Token::try_from(token_pubkey) else {
                continue;
            };

            let base_amount = amount / 10f64.powi(token.decimals as i32);
            let usd_value = base_amount * token_to_price_mapping.get(&token).unwrap();

            token_to_usd_value_mapping.insert(token, usd_value);
            token_to_amount_mapping.insert(token, base_amount);
        }
    }

    let max_value_token = token_to_usd_value_mapping
        .iter()
        .max_by(|(_, val_a), (_, val_b)| val_a.partial_cmp(val_b).unwrap_or(Ordering::Equal))
        .map(|(token, _)| *token)
        .ok_or_else(|| eyre::eyre!("No token with a max value found"))?;

    Ok((max_value_token, *token_to_amount_mapping.get(&max_value_token).unwrap()))
}
