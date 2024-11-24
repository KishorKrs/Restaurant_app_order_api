use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use std::sync::Arc;
use rand::Rng;

use crate::models::{Order, OrderInput, QueryParams};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/orders")
        .route(web::post().to(add_order))
        .route(web::get().to(get_order))
        .route(web::delete().to(remove_order))
    ).service(web::resource("/orders/{table_number}")
    .route(web::get().to(get_orders_for_table)));
}

// add an order
pub async fn add_order(order: web::Json<OrderInput>,pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
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
pub async fn remove_order(info: web::Query<QueryParams>, pool: web::Data<Arc<SqlitePool>>,) -> impl Responder {
    let table_number = info.table_number;
    let order_id = info.order_id;
    let query = "DELETE FROM orders WHERE id = ? AND table_number = ?";
    let result = sqlx::query(query)
        .bind(order_id)
        .bind(table_number)
        .execute(pool.get_ref().as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Order removed successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
