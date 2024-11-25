#[cfg(test)]
mod integration_tests {
    use actix_web::{test, App};
    use serde_json::json;
    use std::{sync::Arc, env};
    use once_cell::sync::Lazy;

    use restaurant_app_order_api::api::config;
    use restaurant_app_order_api::db;
    
    // Shared in-memory SQLite database instance
    static DB_POOL: Lazy<Arc<sqlx::SqlitePool>> = Lazy::new(|| {
        let pool = sqlx::SqlitePool::connect_lazy("sqlite::memory:")
            .expect("Failed to create in-memory SQLite database");
        Arc::new(pool)
    });

    // Function to initialize the database schema
    async fn setup_db() {
        let database_url = ":memory:";
        env::set_var("DATABASE_URL", database_url);
        let pool = DB_POOL.clone();
        db::initialize_database(&pool.clone())
            .await
            .expect("Failed to initialize test database schema");
    }

    #[actix_web::test]
    async fn test_add_order() {
        setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(DB_POOL.clone()))
                .configure(config),
        )
        .await;

        // Simulate a POST request to add an order
        let order_payload = json!({"table_number": 1, "item": "Pizza"});
        let req = test::TestRequest::post()
            .uri("/orders")
            .set_json(&order_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_orders_for_table() {
        setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(DB_POOL.clone()))
                .configure(config),
        )
        .await;

        sqlx::query!("INSERT INTO orders (table_number, item, cook_time) VALUES (?, ?, ?)", 3, "Burger", 12)
            .execute(DB_POOL.as_ref())
            .await
            .unwrap();

        // Simulate a GET request for table 1
        let req = test::TestRequest::get()
            .uri("/orders/3")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Validate the response
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body[0]["table_number"], 3);
        assert_eq!(body[0]["item"], "Burger");
        assert_eq!(body[0]["cook_time"], 12);
    }

    #[actix_web::test]
    async fn test_remove_order() {
        setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(DB_POOL.clone()))
                .configure(config),
        )
        .await;

        sqlx::query!("INSERT INTO orders (table_number, item, cook_time) VALUES (?, ?, ?)", 5, "Sushi", 8)
            .execute(DB_POOL.as_ref())
            .await
            .unwrap();

        // Simulate a DELETE request to remove the order
        let req = test::TestRequest::delete()
            .uri("/orders/3")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Verify that the order is removed
        let remaining_orders = sqlx::query!("SELECT * FROM orders WHERE table_number = ?", 8)
            .fetch_all(DB_POOL.as_ref())
            .await
            .unwrap();
        assert!(remaining_orders.is_empty());
    }
}
