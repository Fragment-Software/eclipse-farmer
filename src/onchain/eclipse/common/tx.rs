use super::constants::ECLIPSE_EXPLORER_URL;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction};

pub async fn send_and_confirm_tx(
    provider: &RpcClient,
    tx: impl SerializableTransaction,
) -> eyre::Result<()> {
    let tx_signature = tx.get_signature();

    tracing::info!("Sending transaction: {}{}", ECLIPSE_EXPLORER_URL, tx_signature);

    match provider.send_and_confirm_transaction(&tx).await {
        Ok(_) => {
            tracing::info!("Transaction confirmed");
        }
        Err(e) => {
            return Err(eyre::eyre!("Failed to send tx: {e}"));
        }
    }

    Ok(())
}
