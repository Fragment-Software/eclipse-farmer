use super::warmup::action::WarmupAction;
use crate::{
    config::Config,
    db::{
        entities::prelude::*,
        service::prelude::{AccountGoalQuery, AccountMutation, AccountQuery},
    },
    onchain::eclipse::{
        common::{token::Token, utils::get_token_with_largest_balance},
        lifinity::swap::swap,
        underdog::create::create_collection,
    },
    utils::misc::{pretty_sleep, random_in_range},
};
use rand::{seq::SliceRandom, thread_rng};
use sea_orm::{ConnectionTrait, DbConn, DbErr};
use solana_client::nonblocking::rpc_client::RpcClient;

use solana_sdk::signer::Signer;
use std::sync::Arc;
use tokio::task::JoinSet;

pub mod action;

pub async fn warmup_mode(connection: DbConn, config: Arc<Config>) -> eyre::Result<()> {
    let spawn_task = |handles: &mut JoinSet<_>,
                      batch: Vec<_>,
                      conn: DbConn,
                      rpc_client: Arc<RpcClient>,
                      config| {
        handles.spawn(async move {
            let thread_res = process_batch(batch.clone(), conn, config, rpc_client).await;
            (batch, thread_res)
        })
    };

    let rpc_client = Arc::new(RpcClient::new(config.general.eclipse_rpc_url.to_string()));
    let thread_count = config.general.thread_count as usize;

    let accounts_ids = match AccountQuery::get_active_accounts(&connection).await {
        Ok(account) => account.into_iter().map(|account| account.id).collect::<Vec<_>>(),
        Err(DbErr::RecordNotFound(_)) => {
            tracing::info!("No more active wallets left!");
            return Ok(());
        }
        Err(e) => eyre::bail!(e),
    };

    let chunk_size = accounts_ids.len() / thread_count +
        if accounts_ids.len() % thread_count != 0 { 1 } else { 0 };

    let mut handles = JoinSet::new();

    for batch in accounts_ids.chunks(chunk_size) {
        let conn = connection.clone();
        let client = rpc_client.clone();
        let batch = batch.to_vec();

        spawn_task(&mut handles, batch, conn, client, config.clone());
    }

    while let Some(res) = handles.join_next().await {
        let (batch_ids, thread_res) = res.unwrap();

        if let Err(e) = thread_res {
            tracing::error!("Thread execution stopped with error: {e}. Restarting the thread...");
            let client = rpc_client.clone();
            let conn = connection.clone();

            spawn_task(&mut handles, batch_ids, conn, client, config.clone());
        }
    }

    tracing::info!("No more active wallets left!");

    Ok(())
}

async fn process_batch<C>(
    batch: Vec<i32>,
    conn: C,
    config: Arc<Config>,
    rpc_client: Arc<RpcClient>,
) -> eyre::Result<()>
where
    C: ConnectionTrait + Clone,
{
    loop {
        let leftover_ids = AccountQuery::get_active_accounts_ids_by_ids(&batch, &conn).await;

        match leftover_ids {
            Ok(mut ids) => {
                ids.shuffle(&mut thread_rng());

                for id in ids {
                    execute_random_warmup_action(
                        conn.clone(),
                        id,
                        rpc_client.clone(),
                        config.clone(),
                    )
                    .await?;
                    pretty_sleep(config.lifinity.wallet_sleep_delay_range, false).await;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}

async fn execute_random_warmup_action<C>(
    conn: C,
    id: i32,
    rpc_client: Arc<RpcClient>,
    config: Arc<Config>,
) -> eyre::Result<()>
where
    C: ConnectionTrait,
{
    let account = AccountQuery::find_account_by_id(id, &conn).await?;
    let account_goal = AccountGoalQuery::get_account_goal_by_id(id, &conn).await?;

    let Some(action) = account.get_random_warmup_action(account_goal) else {
        AccountMutation::mark_as_inactive(id, &conn).await?; // if no action found mark as inactive
        tracing::warn!("{} | Account goal reached, marking as inactive", account.eclipse_pubkey());
        return Ok(());
    };

    match action {
        WarmupAction::LifinitySwap => {
            execute_lifinity_swap(account, rpc_client, conn, config).await
        }
        WarmupAction::UnderdogCreate => {
            execute_underdog_create_nft(account, rpc_client, conn).await
        }
    }
}

async fn execute_lifinity_swap<C>(
    account: AccountModel,
    rpc_client: Arc<RpcClient>,
    conn: C,
    config: Arc<Config>,
) -> eyre::Result<()>
where
    C: ConnectionTrait,
{
    let keypair = account.eclise_keypair();
    let (token_in, balance) =
        get_token_with_largest_balance(&rpc_client, &keypair.pubkey(), None).await?;
    let token_out = Token::get_lifinity_paired_token(&token_in);
    let balance = Token::to_amount(&token_in, balance);

    let percentage = random_in_range(config.lifinity.balance_percentage_range);
    let amount_in = balance * percentage as u64 / 100;

    tracing::info!(
        "{} | Swapping {} {} to {}",
        keypair.pubkey(),
        Token::to_ui_amount(&token_in, amount_in),
        token_in.symbol,
        token_out.symbol
    );
    match swap(&rpc_client, &keypair, &token_in.mint, &token_out.mint, amount_in).await {
        Ok(_) => {
            AccountMutation::increase_swap_count(account.id, &conn).await?;
        }
        Err(e) => tracing::error!("{} | Swap failed: {e}", keypair.pubkey()),
    }

    Ok(())
}

async fn execute_underdog_create_nft<C>(
    account: AccountModel,
    rpc_client: Arc<RpcClient>,
    conn: C,
) -> eyre::Result<()>
where
    C: ConnectionTrait,
{
    let keypair = account.eclise_keypair();
    let proxy = account.proxy();

    tracing::info!("{} | Creating an NFT", keypair.pubkey());
    match create_collection(&rpc_client, &keypair, proxy.as_ref()).await {
        Ok(_) => {
            AccountMutation::increase_create_count(account.id, &conn).await?;
        }
        Err(e) => tracing::error!("{} | Failed to create an NFT: {e}", keypair.pubkey()),
    }

    Ok(())
}
