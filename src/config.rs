use serde::Deserialize;
use std::path::Path;

#[allow(unused)]
const CONFIG_FILE_PATH: &str = "data/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub general: General,
    pub bridge: Bridge,
    pub lifinity: Lifinity,
    pub underdog: Underdog,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct General {
    pub eclipse_rpc_url: String,
    pub mainnet_rpc_url: String,
    pub thread_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Bridge {
    pub balance_percentage_range: [u32; 2],
    pub wallet_sleep_delay_range: [u32; 2],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Lifinity {
    pub swaps_count_range: [u32; 2],
    pub balance_percentage_range: [u32; 2],
    pub wallet_sleep_delay_range: [u32; 2],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Underdog {
    pub create_count_range: [u32; 2],
}

impl Config {
    async fn read_from_file(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let cfg_str = tokio::fs::read_to_string(path).await?;
        Ok(toml::from_str(&cfg_str)?)
    }

    pub async fn read_default() -> Self {
        Self::read_from_file(CONFIG_FILE_PATH).await.expect("Default config to be valid")
    }
}
