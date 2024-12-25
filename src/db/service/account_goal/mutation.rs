use crate::{config::Config, db::entities::prelude::*, utils::misc::random_in_range};

use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Set};

pub struct Mutation;

impl Mutation {
    pub async fn create_account_goal(
        config: &Config,
        account_id: i32,
        connection: &impl ConnectionTrait,
    ) -> Result<i32, DbErr> {
        let swap_count = random_in_range(config.lifinity.swaps_count_range);
        let create_count = random_in_range(config.underdog.create_count_range);

        let account_goal = AccountGoalActiveModel {
            id: Set(account_id),
            swap_count: Set(swap_count as i32),
            create_count: Set(create_count as i32),
        };

        let res = AccountGoal::insert(account_goal).exec(connection).await?;

        Ok(res.last_insert_id)
    }
}
