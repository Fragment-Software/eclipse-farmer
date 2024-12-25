use std::collections::HashMap;

use reqwest::{Method, Proxy};

use crate::{
    onchain::eclipse::common::token::Token,
    utils::fetch::{send_http_request, RequestParams},
};

use super::{constants::TICKER_URL, schemas::TokenInfo};

async fn get_ticker_info(ids: &str, proxy: Option<&Proxy>) -> eyre::Result<Vec<TokenInfo>> {
    let query_args = [("id", ids)].into_iter().collect();

    let request_params = RequestParams {
        url: TICKER_URL,
        method: Method::GET,
        body: None::<serde_json::Value>,
        query_args: Some(query_args),
        proxy,
        headers: None,
    };

    let response_body = send_http_request::<Vec<TokenInfo>>(request_params).await?;

    Ok(response_body)
}

pub async fn get_tickers_usd_value(proxy: Option<&Proxy>) -> eyre::Result<HashMap<Token, f64>> {
    let tickers = get_ticker_info("80,48543,33285", proxy).await?;
    let mut token_to_price_mapping = HashMap::new();

    for ticker in tickers {
        let price = ticker.price_usd.parse::<f64>()?;
        let token = Token::from_coinlore_id(&ticker.id);
        token_to_price_mapping.insert(token, price);
    }

    Ok(token_to_price_mapping)
}
