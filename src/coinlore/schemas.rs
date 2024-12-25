use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenInfo {
    pub id: String,
    pub price_usd: String,
}
