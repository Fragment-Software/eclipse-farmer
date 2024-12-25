use std::{marker::PhantomData, str::FromStr, sync::Arc};

use super::types::Token;
use alloy::{
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
    transports::Transport,
};
use alloy_chains::NamedChain;

sol! {
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    contract IERC20 {
        mapping(address account => uint256) public balanceOf;
    }
}

pub struct EvmClient<P, T>
where
    P: Provider<T, Ethereum>,
    T: Transport + Clone,
{
    provider: Arc<P>,
    chain: NamedChain,
    signer: PrivateKeySigner,
    wallet: EthereumWallet,
    _marker: PhantomData<T>,
}

impl<P, T> EvmClient<P, T>
where
    P: Provider<T, Ethereum>,
    T: Transport + Clone,
{
    pub fn new(provider: Arc<P>, private_key: &str, chain: NamedChain) -> Self {
        let signer = PrivateKeySigner::from_str(private_key).expect("Private key to be valid");
        let wallet = EthereumWallet::new(signer.clone());

        Self { provider, wallet, signer, chain, _marker: PhantomData }
    }

    pub fn address(&self) -> Address {
        self.signer.address()
    }

    pub async fn send_transaction(
        &self,
        to: Address,
        input: Option<Bytes>,
        value: U256,
    ) -> eyre::Result<bool> {
        let eip1559_fees = self.provider.estimate_eip1559_fees(None).await?;

        let nonce = self.provider.get_transaction_count(self.signer.address()).await?;

        let mut tx_request = TransactionRequest::default()
            .with_max_fee_per_gas(eip1559_fees.max_fee_per_gas)
            .with_max_priority_fee_per_gas(eip1559_fees.max_priority_fee_per_gas)
            .with_to(to)
            .with_value(value)
            .with_nonce(nonce)
            .with_chain_id(self.chain as u64)
            .with_from(self.address());

        if let Some(data) = input {
            tx_request.set_input(data);
        }

        let gas_limit = self.provider.estimate_gas(&tx_request).await?;
        tx_request.set_gas_limit(gas_limit);

        let signed_transaction = tx_request.build(&self.wallet).await?;
        let pending_tx = self.provider.send_tx_envelope(signed_transaction).await?;
        let receipt = pending_tx.get_receipt().await?;

        let (_, url) = self.chain.etherscan_urls().unwrap_or(("", ""));

        let tx_status = receipt.status();
        if tx_status {
            tracing::info!("Transaction successful: {url}/tx/{}", receipt.transaction_hash);
        } else {
            tracing::error!("Transaction failed: {url}/tx/{}", receipt.transaction_hash);
        }

        Ok(tx_status)
    }

    pub async fn get_token_balance(
        &self,
        token: Token,
        wallet_address: Option<Address>,
    ) -> eyre::Result<U256> {
        let address = wallet_address.unwrap_or(self.address());

        let balance = match token.is_erc20 {
            true => {
                let contract_instance =
                    IERC20::new(token.contract_address.unwrap(), self.provider.clone());
                contract_instance.balanceOf(address).call().await?._0
            }
            false => self.provider.get_balance(address).await?,
        };

        Ok(balance)
    }
}
