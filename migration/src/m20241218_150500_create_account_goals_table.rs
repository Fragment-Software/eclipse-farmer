use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_table::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccountGoal::Table)
                    .if_not_exists()
                    .col(pk_auto(AccountGoal::Id).integer().not_null())
                    .col(integer(AccountGoal::SwapCount))
                    .col(integer(AccountGoal::CreateCount))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-account-goal-account_id")
                            .from(AccountGoal::Table, AccountGoal::Id)
                            .to(Account::Table, Account::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AccountGoal::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum AccountGoal {
    Table,
    Id,
    SwapCount,
    CreateCount,
}
