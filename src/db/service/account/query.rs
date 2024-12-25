use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use crate::db::entities::{account, prelude::*};

pub struct Query;

impl Query {
    pub async fn find_account_by_id(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<AccountModel, DbErr> {
        if let Some(acc) =
            Account::find().filter(account::Column::Id.eq(account_id)).one(connection).await?
        {
            return Ok(acc);
        }

        Err(DbErr::RecordNotFound(format!("Account with id: {account_id} not found")))
    }

    pub async fn get_active_accounts(
        connection: &impl ConnectionTrait,
    ) -> Result<Vec<AccountModel>, DbErr> {
        let account_vec =
            Account::find().filter(account::Column::IsActive.eq(true)).all(connection).await?;

        match account_vec.is_empty() {
            true => Err(DbErr::RecordNotFound("No active accounts found".to_string())),
            false => Ok(account_vec),
        }
    }

    pub async fn get_active_accounts_ids_by_ids(
        ids: &[i32],
        connection: &impl ConnectionTrait,
    ) -> Result<Vec<i32>, DbErr> {
        let accounts = Account::find()
            .filter(account::Column::IsActive.eq(true).and(account::Column::Id.is_in(ids.to_vec())))
            .all(connection)
            .await?;

        if accounts.is_empty() {
            Err(DbErr::RecordNotFound(
                "No active accounts IDs found for the provided IDs".to_string(),
            ))
        } else {
            Ok(accounts.into_iter().map(|state| state.id).collect())
        }
    }
}
