use crate::db::entities::prelude::*;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel, Set};

pub struct Mutation;

use super::query::Query;

impl Mutation {
    pub async fn set_funds_bridged(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<BridgeStateModel, DbErr> {
        if let Some(state) = Query::get_account_bridge_state_by_id(account_id, connection).await? {
            let mut state = state.into_active_model();
            state.funds_bridged = Set(true);

            state.update(connection).await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Failed to set funds bridge for account with id {account_id}"
            )))
        }
    }

    pub async fn create_account_bridge_state(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<i32, DbErr> {
        let account_bridge_state =
            BridgeStateActiveModel { id: Set(account_id), funds_bridged: Set(false) };

        let res = BridgeModuleState::insert(account_bridge_state).exec(connection).await?;

        Ok(res.last_insert_id)
    }
}
