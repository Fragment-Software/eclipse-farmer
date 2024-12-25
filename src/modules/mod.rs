use crate::{
    config::Config,
    db::{establish_connection, generate::generate_db},
};
use bridge::bridge_mode;
use dialoguer::{theme::ColorfulTheme, Select};
use sea_orm::DbConn;
use std::sync::Arc;
use warmup::warmup_mode;

mod bridge;
pub mod warmup;

const LOGO: &str = r#"
    ___                                                  __
  /'___\                                                /\ \__
 /\ \__/  _ __    __       __     ___ ___      __    ___\ \ ,_\
 \ \ ,__\/\`'__\/'__`\   /'_ `\ /' __` __`\  /'__`\/' _ `\ \ \/
  \ \ \_/\ \ \//\ \L\.\_/\ \L\ \/\ \/\ \/\ \/\  __//\ \/\ \ \ \_
   \ \_\  \ \_\\ \__/.\_\ \____ \ \_\ \_\ \_\ \____\ \_\ \_\ \__\
    \/_/   \/_/ \/__/\/_/\/___L\ \/_/\/_/\/_/\/____/\/_/\/_/\/__/
                  ___  __  /\____/
                /'___\/\ \_\_/__/
   ____    ___ /\ \__/\ \ ,_\ __  __  __     __    _ __    __
  /',__\  / __`\ \ ,__\\ \ \//\ \/\ \/\ \  /'__`\ /\`'__\/'__`\
 /\__, `\/\ \L\ \ \ \_/ \ \ \\ \ \_/ \_/ \/\ \L\.\\ \ \//\  __/
 \/\____/\ \____/\ \_\   \ \__\ \___x___/'\ \__/.\_\ \_\\ \____\
  \/___/  \/___/  \/_/    \/__/\/__//__/   \/__/\/_/\/_/ \/____/

                     t.me/fragment_software
"#;

pub async fn menu() -> eyre::Result<()> {
    let config = Arc::new(Config::read_default().await);
    let mut conn = establish_connection().await?;

    println!("{LOGO}");

    loop {
        let options =
            vec!["Database menu", "Bridge mode (MAINNET -> ECLIPSE)", "Warmup mode", "Exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choice:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                if let Some(connection) = db_menu(config.clone(), conn.clone()).await? {
                    conn = connection
                }
            }
            1 => bridge_mode(conn.clone(), config.clone()).await?,
            2 => warmup_mode(conn.clone(), config.clone()).await?,
            3 => {
                return Ok(());
            }
            _ => tracing::error!("Invalid selection"),
        }
    }
}

async fn db_menu(config: Arc<Config>, conn: DbConn) -> eyre::Result<Option<DbConn>> {
    loop {
        let sub_options =
            vec!["Generate a new database", "Append data to the existing database", "Back"];

        let sub_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choice:")
            .items(&sub_options)
            .default(0)
            .interact()
            .unwrap();

        match sub_selection {
            0 => {
                let conn = establish_connection().await?;
                generate_db(&config, &conn).await?;
                tracing::info!("Database generated successfully");
                return Ok(Some(conn));
            }
            1 => {
                generate_db(&config, &conn).await?;
                tracing::info!("Data added successfully");
            }
            2 => {
                break;
            }
            _ => {
                tracing::error!("Invalid sub-selection.");
            }
        }
    }

    Ok(None)
}
