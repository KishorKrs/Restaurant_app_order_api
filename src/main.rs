use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;
use rand::Rng;

// Order struct to resemble order table
#[derive(Serialize, Deserialize, FromRow)]
struct Order {
    id: Option<i64>,    // Auto-incrementing ID
    table_number: i32,  // Table number
    item: String,       // Name of the menu item
    cook_time: i32,     // Time to cook item (in minutes)
}

// add an order
async fn add_order(order: web::Json<Order>,pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
    let cook_time = rand::thread_rng().gen_range(5..=15); // Generate random number from 5 to 15
    let query = "INSERT INTO orders (table_number, item, cook_time) VALUES (?, ?, ?)";
    let result = sqlx::query(query)
        .bind(order.table_number)
        .bind(&order.item)
        .bind(cook_time)
        .execute(pool.get_ref().as_ref())
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();
            HttpResponse::Ok().json(Order {
                id: Some(id),
                table_number: order.table_number,
                item: order.item.clone(),
                cook_time,
            })
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the SQLite connection pool
    let pool = SqlitePool::connect("sqlite:./orders.db").await.unwrap();
    let shared_pool = Arc::new(pool);

    // Start the Actix Web Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_pool.clone()))
            .route("/orders", web::post().to(add_order))    // Add order
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
