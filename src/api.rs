use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::utils::generate_random_cook_time;
use crate::models::{Order, OrderInput, QueryParams};


// add an order
pub async fn add_order(order: web::Json<OrderInput>,pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
    let cook_time = generate_random_cook_time(); // Generate random number from 5 to 15
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
pub async fn get_orders_for_table(table_number: web::Path<i32>, pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
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
pub async fn get_order(info: web::Query<QueryParams>, pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
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

// remove an order
pub async fn remove_order(info: web::Path<i32>, pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
    let order_id = info.into_inner();
    let query = "DELETE FROM orders WHERE id = ?";
    let result = sqlx::query(query)
        .bind(order_id)
        .execute(pool.get_ref().as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Order removed successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn handle_404() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found: The requested resource does not exist.")
}
