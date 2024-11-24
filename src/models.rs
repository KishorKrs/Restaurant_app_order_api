use sqlx::FromRow;
use serde::{Deserialize, Serialize};

// Order struct to resemble order table
#[derive(Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Option<i64>,    // Auto-incrementing ID
    pub table_number: i32,  // Table number
    pub item: String,       // Name of the menu item
    pub cook_time: i32,     // Time to cook item (in minutes)
}

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    pub table_number: i32,
    pub item: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub table_number: i32,
    pub order_id: i64,
}
