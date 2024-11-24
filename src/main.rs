use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

// Order struct to resemble order table
#[derive(Serialize, Deserialize, FromRow)]
struct Order {
    id: Option<i64>,    // Auto-incrementing ID
    table_number: i32,  // Table number
    item: String,       // Name of the menu item
    cook_time: i32,     // Time to cook item (in minutes)
}


fn main() {
    println!("Hello, world!");
}
