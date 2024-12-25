pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241218_150500_create_account_goals_table;
mod m20241222_135418_create_bridge_goals_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241218_150500_create_account_goals_table::Migration),
            Box::new(m20241222_135418_create_bridge_goals_table::Migration),
        ]
    }
}
