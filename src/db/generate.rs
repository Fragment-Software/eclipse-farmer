use sea_orm::{DbConn, DbErr};

use crate::{
    config::Config,
    db::constants::{
        ECLIPSE_PRIVATE_KEYS_FILE_PATH, EVM_PRIVATE_KEYS_FILE_PATH, PROXIES_FILE_PATH,
    },
    utils::files::read_file_lines,
};

use super::service::prelude::{AccountGoalMutation, AccountMutation, BridgeModuleStateMutation};

pub async fn generate_db(config: &Config, connection: &DbConn) -> Result<(), DbErr> {
    let (evm_pks, eclipse_pks, proxies) = tokio::try_join!(
        read_file_lines(EVM_PRIVATE_KEYS_FILE_PATH),
        read_file_lines(ECLIPSE_PRIVATE_KEYS_FILE_PATH),
        read_file_lines(PROXIES_FILE_PATH)
    )
    .map_err(|e| DbErr::Custom(format!("Failed to read files: {e}")))?;

    assert_eq!(
        evm_pks.len(),
        eclipse_pks.len(),
        "Amount of EVM and Eclipse private keys is not equal"
    );

    let mut proxies_iter = proxies.into_iter();

    for (evm_pk, eclipse_pk) in evm_pks.into_iter().zip(eclipse_pks.into_iter()) {
        let maybe_proxy = proxies_iter.next();

        match AccountMutation::create_account(&evm_pk, &eclipse_pk, maybe_proxy, connection).await {
            Ok(account_id) => {
                AccountGoalMutation::create_account_goal(config, account_id, connection).await?;
                BridgeModuleStateMutation::create_account_bridge_state(account_id, connection)
                    .await?;
            }
            Err(e) => {
                if let DbErr::Exec(e) = e {
                    if e.to_string().contains("UNIQUE constraint failed") {
                        tracing::warn!(
                                "EVM private key: {:?} or Eclipse private key: {:?} already exists in the database",
                                evm_pk,
                                eclipse_pk,
                            );
                    }
                } else {
                    tracing::error!("{e}")
                }
            }
        }
    }

    if proxies_iter.len() > 0 {
        tracing::warn!("There are {} unused proxy entries left in the file.", proxies_iter.len());
    }

    Ok(())
}
