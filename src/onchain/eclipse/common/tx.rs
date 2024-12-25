use super::constants::ECLIPSE_EXPLORER_URL;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction,
    rpc_config::RpcSendTransactionConfig,
};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_transaction_status::UiTransactionEncoding;

pub async fn send_and_confirm_tx(
    provider: &RpcClient,
    tx: impl SerializableTransaction,
) -> eyre::Result<()> {
    let tx_config = RpcSendTransactionConfig {
        skip_preflight: false,
        preflight_commitment: Some(CommitmentLevel::Confirmed),
        encoding: Some(UiTransactionEncoding::Base64),
        max_retries: None,
        min_context_slot: None,
    };

    match provider.send_transaction_with_config(&tx, tx_config).await {
        Ok(tx_signature) => {
            tracing::info!("Sent transaction: {}{}", ECLIPSE_EXPLORER_URL, tx_signature);

            match provider
                .confirm_transaction_with_commitment(&tx_signature, CommitmentConfig::confirmed())
                .await
            {
                Ok(_) => {
                    tracing::info!("Transaction confirmed");
                }

                Err(e) => {
                    return Err(eyre::eyre!("Transaction failed: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(eyre::eyre!("Failed to send tx: {e}"));
        }
    }

    Ok(())
}
