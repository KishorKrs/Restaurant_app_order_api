use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::sync::Arc;
use crate::api::{add_order, get_order, get_orders_for_table, remove_order};

mod db;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::initialize_database()
        .await
        .map_err(|e| {
            eprintln!("Database initialization failed: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

    // Initialize the SQLite connection pool
    // let pool = SqlitePool::connect(&database_url).await.unwrap();
    let shared_pool = Arc::new(pool);

    // Start the Actix Web Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_pool.clone()))
            .route("/orders", web::post().to(add_order))
            .route("/orders/{table_number}", web::get().to(get_orders_for_table))
            .route("/order", web::get().to(get_order))
            .route("/orders", web::delete().to(remove_order))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
