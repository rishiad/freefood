use dotenv::dotenv;
use tracing::{event, Level};
use std::io::Error as E;
use std::env;

pub fn load_config() -> Result<(), E> {
    event!(Level::INFO, "Loading config...");
    let env_loader = dotenv();

   match env_loader {
    Ok(_) => event!(Level::INFO, "Config loaded successfully"),
    Err(e) => event!(Level::ERROR, "Failed to load config from .env: {}", e)
    }
    Ok(())
}

pub async fn connect_to_db() -> Result<sqlx::MySqlPool, E> {
    event!(Level::INFO, "Connecting to MySQL Database...");
    let mysql_conn = sqlx::mysql::MySqlPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found")).await;

    match &mysql_conn {
        Ok(_) => event!(Level::INFO, "Connected to MySQL Database successfully"),
        Err(e) => event!(Level::ERROR, "Failed to connect to MySQL Database: {}", e)
    }
    Ok(mysql_conn.unwrap())
}
