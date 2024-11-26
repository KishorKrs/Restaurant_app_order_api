use actix_web::{web, App, HttpServer};
use reqwest::blocking::Client;
use std::{sync::Arc, thread, env};
use dotenvy::dotenv;

pub mod api;
pub mod db;
pub mod models;
pub mod utils;
pub mod route;

// Function to simulate a single client making multiple requests
fn simulate_client(client_id: usize, base_url: &str) {
    let client = Client::new();

    // POST Order
    let add_order_resp = client
        .post(format!("{}/orders", base_url))
        .json(&serde_json::json!({
            "table_number": client_id,
            "item": format!("Dish {}", client_id)
        }))
        .send();

    match add_order_resp {
        Ok(response) => println!("Client {} added order: Status {}", client_id, response.status()),
        Err(err) => println!("Client {} failed to add order: {}", client_id, err),
    }

    // Query Table Order
    let query_resp = client
        .get(format!("{}/orders/{}", base_url, client_id))
        .send();

    match query_resp {
        Ok(response) => println!("Client {} Table Query: {:?}", client_id, response.text().unwrap()),
        Err(err) => println!("Client {} failed to Table Query: {}", client_id, err),
    }

    // Query Specific Order
    let get_query_resp = client
        .get(format!(
            "{}/orders?table_number={}&order_id={}",
            base_url, client_id, client_id
        ))
        .send();

    match get_query_resp {
        Ok(response) => println!("Client {} Query Order: {:?}", client_id, response.text().unwrap()),
        Err(err) => println!("Client {} failed to Query Order: {}", client_id, err),
    }

    // Delete an Order
    let delete_resp = client
        .delete(format!("{}/orders/{}", base_url, client_id))
        .send();

    match delete_resp {
        Ok(response) => println!("Client {} deleted order: Status {}", client_id, response.status()),
        Err(err) => println!("Client {} failed to delete order: {}", client_id, err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("PORT must be set");
    let port_num = port.parse::<u16>().unwrap();

    // Initialize the SQLite connection pool
    let pool = db::get_pool().await.expect("Failed to get the database pool");
    let shared_pool = Arc::new(pool);
    db::initialize_database(&shared_pool).await.map_err(|e| {
        eprintln!("Database initialization failed: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    
    // Start the Actix Web Server in a separate thread
    let server_thread = thread::spawn(move || {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(shared_pool.clone()))
                .configure(route::config)
        })
        .bind(("127.0.0.1", port_num))
        .expect("Failed to bind server")
        .run();

        let rt = actix_rt::System::new();
        rt.block_on(server)
    });

    // Simulate multiple clients in the main thread
    let client_threads: Vec<_> = (1..=10)
        .map(|id| {
            let url = format!("http://127.0.01:{}", port);
            thread::spawn(move || simulate_client(id, &url))
        })
        .collect();

    // Wait for all client threads to complete
    for handle in client_threads {
        handle.join().unwrap();
    }

    let _ = server_thread.join().unwrap();
    Ok(())
}
