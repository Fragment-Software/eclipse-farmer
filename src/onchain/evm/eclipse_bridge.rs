use alloy::{
    network::Ethereum,
    primitives::{FixedBytes, U256},
    providers::Provider,
    sol,
    sol_types::SolCall,
    transports::Transport,
};
use solana_sdk::pubkey::Pubkey;

use super::{client::EvmClient, constants::ECLIPSE_BRIDGE_CONTRACT_ADDRESS};

sol! {
    /// @inheritdoc ICanonicalBridge
    /// @dev Access controlled, pausible
    function deposit(bytes32 recipient, uint256 amountWei)
        external
        payable
        virtual
        override
        whenNotPaused
        bytes32Initialized(recipient)
        validDepositAmount(amountWei)
        nonReentrant;
}

/// Returns `Ok(true)` in case of a confirmed successful transaction
pub async fn deposit<P, T>(
    client: EvmClient<P, T>,
    recipient: Pubkey,
    amount: U256,
) -> eyre::Result<bool>
where
    P: Provider<T, Ethereum>,
    T: Transport + Clone,
{
    let recipient = FixedBytes::from_slice(&recipient.to_bytes());

    let input = depositCall { recipient, amountWei: amount }.abi_encode();

    client.send_transaction(ECLIPSE_BRIDGE_CONTRACT_ADDRESS, Some(input.into()), amount).await
}
