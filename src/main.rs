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

#[derive(Debug, Deserialize)]
struct OrderInput {
    table_number: i32,
    item: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    table_number: i32,
    order_id: i64,
}

// add an order
async fn add_order(order: web::Json<OrderInput>,pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
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

// get all orders for a table
async fn get_orders_for_table(table_number: web::Path<i32>, pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
    let query = "SELECT id, table_number, item, cook_time FROM orders WHERE table_number = ?";
    let rows = sqlx::query_as::<_, Order>(query)
        .bind(table_number.into_inner())
        .fetch_all(pool.get_ref().as_ref())
        .await;

    match rows {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// get a specific order for a table
async fn get_order(info: web::Query<QueryParams>, pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
    let table_number = info.table_number;
    let order_id = info.order_id;
    let query = "SELECT id, table_number, item, cook_time FROM orders WHERE id = ? AND table_number = ?";
    let row = sqlx::query_as::<_, Order>(query)
        .bind(order_id)
        .bind(table_number)
        .fetch_optional(pool.get_ref().as_ref())
        .await;

    match row {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => HttpResponse::NotFound().body("Order not found"),
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
            .route("/orders/{table_number}", web::get().to(get_orders_for_table))
            .route("/order", web::get().to(get_order))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
