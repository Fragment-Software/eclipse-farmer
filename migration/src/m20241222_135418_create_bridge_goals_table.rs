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
                    .table(BridgeModuleState::Table)
                    .if_not_exists()
                    .col(pk_auto(BridgeModuleState::Id).integer().not_null())
                    .col(boolean(BridgeModuleState::FundsBridged))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bridge-module-state-account_id")
                            .from(BridgeModuleState::Table, BridgeModuleState::Id)
                            .to(Account::Table, Account::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(BridgeModuleState::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum BridgeModuleState {
    Table,
    Id,
    FundsBridged,
}
