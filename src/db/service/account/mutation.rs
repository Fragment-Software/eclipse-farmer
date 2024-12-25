use std::str::FromStr;

use crate::db::{entities::prelude::*, service::prelude::*};
use alloy::signers::local::PrivateKeySigner;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    Set,
};
use solana_sdk::{signature::Keypair, signer::Signer};

pub struct Mutation;

impl Mutation {
    pub async fn create_account(
        evm_private_key: &str,
        eclipse_private_key: &str,
        proxy: Option<String>,
        connection: &impl ConnectionTrait,
    ) -> Result<i32, DbErr> {
        let evm_pk = PrivateKeySigner::from_str(evm_private_key).map_err(|e| {
            DbErr::Custom(format!("Invalid EVM private key `{evm_private_key}`: {e}"))
        })?;

        let pk_bytes = &solana_sdk::bs58::decode(&eclipse_private_key).into_vec().map_err(|e| {
            DbErr::Custom(format!("Invalid base58 string `{eclipse_private_key}`: {e}"))
        })?;
        let eclipse_pk = Keypair::from_bytes(pk_bytes).map_err(|e| {
            DbErr::Custom(format!("Invalid Eclipse private key {eclipse_private_key}: {e}"))
        })?;

        let account = AccountActiveModel {
            id: NotSet,
            is_active: Set(true),
            evm_private_key: Set(evm_private_key.to_string()),
            evm_address: Set(evm_pk.address().to_string()),
            eclipse_private_key: Set(eclipse_private_key.to_string()),
            eclipse_address: Set(eclipse_pk.pubkey().to_string()),
            proxy: Set(proxy),
            swap_count: Set(0),
            create_count: Set(0),
        };

        let res = Account::insert(account).exec(connection).await?;

        Ok(res.last_insert_id)
    }

    pub async fn increase_swap_count(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<AccountModel, DbErr> {
        let acc = AccountQuery::find_account_by_id(account_id, connection).await?;
        let new_swap_count = Set(acc.swap_count + 1);

        let mut account = acc.into_active_model();
        account.swap_count = new_swap_count;

        account.update(connection).await
    }

    pub async fn increase_create_count(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<AccountModel, DbErr> {
        let acc = AccountQuery::find_account_by_id(account_id, connection).await?;
        let new_create_count = Set(acc.create_count + 1);

        let mut account = acc.into_active_model();
        account.create_count = new_create_count;

        account.update(connection).await
    }

    pub async fn mark_as_inactive(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<AccountModel, DbErr> {
        let acc = AccountQuery::find_account_by_id(account_id, connection).await?;

        let mut account = acc.into_active_model();
        account.is_active = Set(false);

        account.update(connection).await
    }
}
