mod config;
mod repositories;
mod models;
mod dal;

use actix_web::{web, App, HttpResponse, HttpServer};
use std::{env, io::Result};
use tracing::{event, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing Subscriber
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("Failed to set tracing subscriber");

    // Load config
    config::load_config().expect("Failed to load config");

        // Connect to MySQL Database
    config::connect_to_db().await.expect("Failed to connect to MySQL Database");
    // Start server
    event!(
        Level::INFO,
        "Starting server at port {}",
        env::var("PORT").unwrap()
    );
    
    HttpServer::new(move || {
        event!(Level::INFO, "Regestering routes");
        App::new().route(
            "/",
            web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }),
        )
    })
    .bind(format!(
        "{}:{}",
        env::var("HOST").unwrap(),
        env::var("PORT").unwrap()
    ))
    .expect("msg")
    .run()
    .await?;



    Ok(())
}
