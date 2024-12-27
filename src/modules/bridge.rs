use std::sync::Arc;

use crate::{
    config::Config,
    db::service::prelude::*,
    onchain::evm::{eclipse_bridge::deposit, types::Token},
    utils::misc::{pretty_sleep, random_in_range},
};
use alloy::{
    network::Ethereum,
    primitives::{
        utils::{format_units, parse_ether},
        U256,
    },
    providers::{builder, Provider, RootProvider},
    rpc::client::ClientBuilder,
    transports::{
        http::Http,
        layers::{RetryBackoffLayer, RetryBackoffService},
        Transport,
    },
};
use rand::{seq::SliceRandom, thread_rng};
use reqwest::Client;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbConn, DbErr};
use tokio::task::JoinSet;

pub async fn bridge_mode(connection: DbConn, config: Arc<Config>) -> eyre::Result<()> {
    let spawn_task = |handles: &mut JoinSet<_>,
                      batch_ids: Vec<_>,
                      conn: DatabaseConnection,
                      provider: Arc<RootProvider<RetryBackoffService<Http<Client>>>>,
                      balance_range: [u32; 2],
                      sleep_range: [u32; 2]| {
        handles.spawn(async move {
            let thread_res =
                process_batch(batch_ids.clone(), conn, provider, balance_range, sleep_range).await;
            (batch_ids, thread_res)
        })
    };

    let retry_layer = RetryBackoffLayer::new(10, 2, 500);
    let client = ClientBuilder::default()
        .layer(retry_layer)
        .transport(Http::new(config.general.mainnet_rpc_url.parse()?), false);
    let provider = Arc::new(builder::<Ethereum>().on_provider(RootProvider::new(client)));

    let thread_count = config.general.thread_count as usize;
    let balance_range = config.bridge.balance_percentage_range;
    let sleep_delay = config.bridge.wallet_sleep_delay_range;

    let accounts_states_ids =
        match BridgeModuleStateQuery::get_accounts_with_unbridged_state(&connection).await {
            Ok(states) => states.into_iter().map(|state| state.id).collect::<Vec<_>>(),
            Err(DbErr::RecordNotFound(_)) => {
                tracing::info!("Funds were bridged for all wallets!");
                return Ok(());
            }
            Err(e) => eyre::bail!(e),
        };

    let chunk_size = accounts_states_ids.len() / thread_count
        + if accounts_states_ids.len() % thread_count != 0 { 1 } else { 0 };

    let mut handles = JoinSet::new();

    for batch in accounts_states_ids.chunks(chunk_size) {
        let conn = connection.clone();
        let provider = provider.clone();
        let batch_ids = batch.to_vec();

        spawn_task(&mut handles, batch_ids, conn, provider, balance_range, sleep_delay);
    }

    while let Some(res) = handles.join_next().await {
        let (batch_ids, thread_res) = res.unwrap();

        if let Err(e) = thread_res {
            tracing::error!("Thread execution stopped with error: {e}. Restarting the thread...");
            let provider = provider.clone();
            let conn = connection.clone();
            spawn_task(&mut handles, batch_ids, conn, provider, balance_range, sleep_delay);
        }
    }

    tracing::info!("Funds were bridged for all wallets!");

    Ok(())
}

async fn process_batch<C, P, T>(
    batch: Vec<i32>,
    conn: C,
    provider: Arc<P>,
    balance_range: [u32; 2],
    sleep_range: [u32; 2],
) -> eyre::Result<()>
where
    C: ConnectionTrait + Clone,
    P: Provider<T, Ethereum>,
    T: Transport + Clone,
{
    loop {
        let leftover_batch_ids =
            BridgeModuleStateQuery::get_unbridged_state_ids_by_ids(&batch, &conn).await;

        match leftover_batch_ids {
            Ok(mut ids) => {
                ids.shuffle(&mut thread_rng());

                for id in ids {
                    bridge_funds(conn.clone(), id, provider.clone(), balance_range).await?;
                    pretty_sleep(sleep_range, false).await;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}

async fn bridge_funds<C, P, T>(
    conn: C,
    id: i32,
    provider: Arc<P>,
    balance_range: [u32; 2],
) -> eyre::Result<()>
where
    C: ConnectionTrait,
    P: Provider<T, Ethereum>,
    T: Transport + Clone,
{
    let account = AccountQuery::find_account_by_id(id, &conn).await?;
    let client = account.to_evm_client(provider);
    let client_address = client.address();
    let eclipse_pubkey = account.eclipse_pubkey();

    let balance = client.get_token_balance(Token::ETH, None).await?;
    let percentage = random_in_range(balance_range);
    let mut amount = balance * U256::from(percentage) / U256::from(100);
    let divisor = U256::from(10).pow(U256::from(11));
    amount = (amount / divisor) * divisor;
    let ui_amount = format_units(amount, 18)?;

    let min_amount = parse_ether("0.002")?;
    if amount < min_amount {
        tracing::warn!(
            "Amount is lower than min bridge amount: {ui_amount} < {}",
            format_units(min_amount, 18)?
        );
        amount = min_amount;
    }

    tracing::info!("{client_address} | Bridging {ui_amount} ETH to {eclipse_pubkey}");

    match deposit(client, eclipse_pubkey, amount).await {
        Ok(res) => {
            match res {
                true => {
                    BridgeModuleStateMutation::set_funds_bridged(id, &conn).await?;
                    tracing::info!("{} | Bridge sent successfully", client_address)
                }
                false => eyre::bail!("Transaction was sent but failed"),
            };
        }
        Err(e) => tracing::error!("Failed to send a transaction {e}"),
    }

    Ok(())
}
