use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id).integer().not_null())
                    .col(boolean(Account::IsActive))
                    .col(string(Account::EvmPrivateKey).unique_key())
                    .col(string(Account::EvmAddress).unique_key())
                    .col(string(Account::EclipsePrivateKey).unique_key())
                    .col(string(Account::EclipseAddress).unique_key())
                    .col(ColumnDef::new(Account::Proxy).string())
                    .col(integer(Account::SwapCount))
                    .col(integer(Account::CreateCount))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Account::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    Id,
    IsActive,
    EvmPrivateKey,
    EvmAddress,
    EclipsePrivateKey,
    EclipseAddress,
    Proxy,
    SwapCount,
    CreateCount,
}
