mod event_dal_test;

use crate::config;
use sqlx::{self, Connection};

async fn setup() -> Result<(sqlx::Transaction<'static, sqlx::MySql>, sqlx::Pool<sqlx::MySql>), sqlx::Error> {
    config::load_config;
    let pool = sqlx::mysql::MySqlPool::connect("mysql://t5bn87dbn057k74uus85:pscale_pw_WqNGiOfs3Lm51Zn1oPPz2VA0URtW3CLUaFgODVLqOZt@aws.connect.psdb.cloud/arounddb").await?;
    let tx = pool.begin().await?;

    Ok((tx, pool))
}

async fn teardown(tx: sqlx::Transaction<'static, sqlx::MySql>) -> Result<(), sqlx::Error> {
    tx.rollback().await?;
    Ok(())
}