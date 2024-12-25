use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use crate::db::entities::{
    bridge_module_state,
    prelude::{BridgeModuleState, BridgeStateModel},
};

pub struct Query;

impl Query {
    pub async fn get_accounts_with_unbridged_state(
        connection: &impl ConnectionTrait,
    ) -> Result<Vec<BridgeStateModel>, DbErr> {
        let state_vec = BridgeModuleState::find()
            .filter(bridge_module_state::Column::FundsBridged.eq(false))
            .all(connection)
            .await?;

        match state_vec.is_empty() {
            true => {
                Err(DbErr::RecordNotFound("No accounts with not bridged assets found".to_string()))
            }
            false => Ok(state_vec),
        }
    }

    pub async fn get_unbridged_state_ids_by_ids(
        ids: &[i32],
        connection: &impl ConnectionTrait,
    ) -> Result<Vec<i32>, DbErr> {
        let states = BridgeModuleState::find()
            .filter(
                bridge_module_state::Column::FundsBridged
                    .eq(false)
                    .and(bridge_module_state::Column::Id.is_in(ids.to_vec())),
            )
            .all(connection)
            .await?;

        if states.is_empty() {
            Err(DbErr::RecordNotFound(
                "No unbridged state IDs found for the provided IDs".to_string(),
            ))
        } else {
            Ok(states.into_iter().map(|state| state.id).collect())
        }
    }

    pub async fn get_account_bridge_state_by_id(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<Option<BridgeStateModel>, DbErr> {
        BridgeModuleState::find()
            .filter(bridge_module_state::Column::Id.eq(account_id))
            .one(connection)
            .await
    }
}
