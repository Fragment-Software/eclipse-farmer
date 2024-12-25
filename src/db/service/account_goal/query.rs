use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use crate::db::entities::{account_goal, prelude::*};

pub struct Query;

impl Query {
    pub async fn get_account_goal_by_id(
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<AccountGoalModel, DbErr> {
        let goal = AccountGoal::find()
            .filter(account_goal::Column::Id.eq(account_id))
            .one(connection)
            .await?
            .unwrap();

        Ok(goal)
    }
}
