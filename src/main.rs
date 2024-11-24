use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::sync::Arc;

// NOTE: All modules should be include here even though its not directly used in main
mod db;
mod api;
mod utils;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize the SQLite connection pool
    let pool = db::initialize_database()
        .await
        .map_err(|e| {
            eprintln!("Database initialization failed: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

    let shared_pool = Arc::new(pool);

    // Start the Actix Web Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_pool.clone()))
            .configure(api::config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
