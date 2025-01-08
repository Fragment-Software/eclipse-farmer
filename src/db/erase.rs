use sea_orm::{ConnectionTrait, DatabaseBackend, DbConn, DbErr, Statement};

pub async fn erase_all_tables(db: &DbConn) -> Result<(), DbErr> {
    db.execute(Statement::from_string(DatabaseBackend::Sqlite, "DELETE FROM account_goal;"))
        .await?;
    db.execute(Statement::from_string(DatabaseBackend::Sqlite, "DELETE FROM bridge_module_state;"))
        .await?;

    db.execute(Statement::from_string(DatabaseBackend::Sqlite, "DELETE FROM account;")).await?;

    // reset id ordering
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        "DELETE FROM sqlite_sequence WHERE name='account_goal';",
    ))
    .await?;
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        "DELETE FROM sqlite_sequence WHERE name='bridge_module_state';",
    ))
    .await?;
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        "DELETE FROM sqlite_sequence WHERE name='account';",
    ))
    .await?;

    Ok(())
}
