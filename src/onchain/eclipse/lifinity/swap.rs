use borsh::BorshDeserialize;
use eyre::bail;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_response::{Response, RpcSimulateTransactionResult},
};
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::Transaction,
};

use crate::onchain::eclipse::{
    common::{
        constants::{ETH_PUBKEY, TOKEN_2022_PROGRAM_ID},
        derive::{derive_ata, derive_program_authority},
        ixs::{create_ata, sync_native, unwrap_eth},
        state::{Account, Amm},
        tx::send_and_confirm_tx,
        typedefs::CreateAtaArgs,
    },
    lifinity::utils::extract_out_value,
};

use super::{
    math::calculate_min_amount_out,
    typedefs::{LifinitySwapArgs, SwapInfo},
    utils::{assemble_swap_ix, determine_pool_pubkey, determine_trade_direction},
};

pub async fn prepare_swap(
    provider: &RpcClient,
    swap_info: SwapInfo,
) -> eyre::Result<Vec<Instruction>> {
    let mut ixs = vec![];

    let (authority, _) = derive_program_authority(&swap_info.amm_pool_pubkey);
    let (source_info, _) =
        derive_ata(&swap_info.wallet_pubkey, &swap_info.token_a, &TOKEN_2022_PROGRAM_ID);

    ixs.push(create_ata(CreateAtaArgs {
        funding_address: swap_info.wallet_pubkey,
        associated_account_address: source_info,
        wallet_address: swap_info.wallet_pubkey,
        token_mint_address: swap_info.token_a,
        token_program_id: TOKEN_2022_PROGRAM_ID,
        instruction: 1,
    }));

    if swap_info.should_transfer_source {
        ixs.push(solana_sdk::system_instruction::transfer(
            &swap_info.wallet_pubkey,
            &source_info,
            swap_info.amount_in,
        ));

        ixs.push(sync_native(&TOKEN_2022_PROGRAM_ID, &source_info));
    }

    let (destination_info, _) =
        derive_ata(&swap_info.wallet_pubkey, &swap_info.token_b, &TOKEN_2022_PROGRAM_ID);

    ixs.push(create_ata(CreateAtaArgs {
        funding_address: swap_info.wallet_pubkey,
        associated_account_address: destination_info,
        wallet_address: swap_info.wallet_pubkey,
        token_mint_address: swap_info.token_b,
        token_program_id: TOKEN_2022_PROGRAM_ID,
        instruction: 1,
    }));

    let amm_data = provider.get_account_data(&swap_info.amm_pool_pubkey).await?;
    let amm = Amm::deserialize(&mut &amm_data[8..])?;

    let (account_a_result, account_b_result) = tokio::join!(
        provider.get_account_data(&amm.token_a_account),
        provider.get_account_data(&amm.token_b_account),
    );

    let token_a_reserves = match account_a_result {
        Ok(account_a_data) => {
            let account = Account::deserialize(&account_a_data)?;
            account.amount
        }
        Err(e) => {
            bail!("Failed to get token A reserves: {e}")
        }
    };

    let token_b_reserves = match account_b_result {
        Ok(account_b_data) => {
            let account = Account::deserialize(&account_b_data)?;
            account.amount
        }
        Err(e) => {
            bail!("Failed to get token B reserves: {e}")
        }
    };

    let trade_direction = determine_trade_direction(
        &swap_info.amm_pool_pubkey,
        &swap_info.token_a,
        &swap_info.token_b,
    )?;

    let amount_out = match swap_info.amount_out {
        Some(value) => value - (value * 2 / 100),
        None => calculate_min_amount_out(
            swap_info.amount_in,
            token_a_reserves,
            token_b_reserves,
            trade_direction,
        ),
    };

    let (swap_source, swap_destination, source_mint, destination_mint) =
        if swap_info.token_a == amm.token_a_mint {
            (amm.token_a_account, amm.token_b_account, amm.token_a_mint, amm.token_b_mint)
        } else if swap_info.token_a == amm.token_b_mint {
            (amm.token_b_account, amm.token_a_account, amm.token_b_mint, amm.token_a_mint)
        } else {
            eyre::bail!("Invalid amm pool");
        };

    let args = LifinitySwapArgs {
        authority,
        amm: swap_info.amm_pool_pubkey,
        user_transfer_authority: swap_info.wallet_pubkey,
        source_info,
        destination_info,
        swap_source,
        swap_destination,
        source_mint,
        destination_mint,
        pool_mint: amm.pool_mint,
        fee_account: amm.fee_account,
        token_program: TOKEN_2022_PROGRAM_ID,
        oracle_main_account: amm.oracle_main_account,
        oracle_sub_account: amm.oracle_sub_account,
        oracle_pc_account: amm.oracle_pc_account,
        amount_in: swap_info.amount_in,
        minimum_amount_out: amount_out,
    };

    ixs.push(assemble_swap_ix(args));

    if swap_info.token_b == ETH_PUBKEY {
        ixs.extend_from_slice(&unwrap_eth(&swap_info.wallet_pubkey, &destination_info));
    }

    Ok(ixs)
}

async fn simulate_transaction(
    provider: &RpcClient,
    wallet_kp: &Keypair,
    swap_info: SwapInfo,
) -> eyre::Result<Response<RpcSimulateTransactionResult>> {
    let swap_ixs = prepare_swap(provider, swap_info).await?;

    let (recent_blockhash, _) =
        provider.get_latest_blockhash_with_commitment(CommitmentConfig::finalized()).await?;

    let tx = Transaction::new_signed_with_payer(
        &swap_ixs,
        Some(&wallet_kp.pubkey()),
        &[wallet_kp],
        recent_blockhash,
    );

    let simulation = provider.simulate_transaction(&tx).await?;

    Ok(simulation)
}

async fn execute_transaction(
    provider: &RpcClient,
    wallet_kp: &Keypair,
    swap_info: SwapInfo,
) -> eyre::Result<()> {
    let swap_ixs = prepare_swap(provider, swap_info).await?;

    let (recent_blockhash, _) =
        provider.get_latest_blockhash_with_commitment(CommitmentConfig::finalized()).await?;

    let tx = Transaction::new_signed_with_payer(
        &swap_ixs,
        Some(&wallet_kp.pubkey()),
        &[wallet_kp],
        recent_blockhash,
    );

    send_and_confirm_tx(provider, tx).await
}

pub async fn swap(
    provider: &RpcClient,
    wallet_kp: &Keypair,
    token_a: &Pubkey,
    token_b: &Pubkey,
    amount_in: u64,
) -> eyre::Result<()> {
    let amm_pool_key = match determine_pool_pubkey(token_a, token_b) {
        Some(pubkey) => pubkey,
        None => eyre::bail!("Failed to determine pool pubkey"),
    };

    let should_transfer_source = *token_a == ETH_PUBKEY;

    let simulate_swap_info = SwapInfo::new(
        &wallet_kp.pubkey(),
        token_a,
        token_b,
        &amm_pool_key,
        amount_in,
        None,
        should_transfer_source,
    );

    let simulation = simulate_transaction(provider, wallet_kp, simulate_swap_info).await?;

    let logs = simulation.value.logs.unwrap();

    let amount_out = logs.iter().filter_map(|log| extract_out_value(log)).next().unwrap_or(0);

    let swap_info = SwapInfo::new(
        &wallet_kp.pubkey(),
        token_a,
        token_b,
        &amm_pool_key,
        amount_in,
        Some(amount_out),
        should_transfer_source,
    );

    execute_transaction(provider, wallet_kp, swap_info).await?;

    Ok(())
}
