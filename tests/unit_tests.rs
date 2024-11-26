#[cfg(test)]
mod unit_tests {
    use sqlx::SqlitePool;
    use std::sync::Arc;
    use serde_json::json;
    use actix_web::{web, http::StatusCode, test};

    use restaurant_app_order_api::db;
    use restaurant_app_order_api::route;
    use restaurant_app_order_api::utils;
    use restaurant_app_order_api::models;

    // Helper function to set up an in-memory test database
    async fn setup_test_db() -> web::Data<Arc<SqlitePool>> {
        let database_url = ":memory:";
        std::env::set_var("DATABASE_URL", database_url);
        let pool = db::get_pool().await.expect("Failed to initialize pool");
        db::initialize_database(&pool)
            .await
            .expect("Failed to initialize database");
        web::Data::new(Arc::new(pool))
    }

    #[actix_web::test]
    async fn test_add_order() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(pool.clone())
                .configure(route::config),
        )
        .await;
    
        let payload = json!({"table_number": 1, "item": "Pizza"});
        let req = test::TestRequest::post()
            .uri("/orders")
            .set_json(&payload)
            .to_request();
    
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    
        let body: models::Order = test::read_body_json(resp).await;
        assert_eq!(body.table_number, 1);
        assert_eq!(body.item, "Pizza");
        assert!(body.cook_time >= 5 && body.cook_time <= 15);
    }

    #[actix_web::test]
    async fn test_get_order() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
            .app_data(pool.clone())
            .configure(route::config)
        )
        .await;

        // Insert test data
        sqlx::query!("INSERT INTO orders (id, table_number, item, cook_time) VALUES (?, ?, ?, ?)", 100, 10, "Burger", 10)
            .execute(pool.get_ref().as_ref())
            .await
            .expect("Failed to insert order");

        let req = test::TestRequest::get().uri("/orders?table_number=10&order_id=100").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: models::Order = test::read_body_json(resp).await;
        assert_eq!(body.item, "Burger");
        assert_eq!(body.cook_time, 10);
        assert_eq!(body.table_number, 10);
    }

    #[actix_web::test]
    async fn test_get_orders_for_table() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
            .app_data(pool.clone())
            .configure(route::config),
        )
        .await;

        // Insert test data
        sqlx::query!("INSERT INTO orders (table_number, item, cook_time) VALUES (?, ?, ?)", 1, "Burger", 10)
            .execute(pool.get_ref().as_ref())
            .await
            .expect("Failed to insert order");

        let req = test::TestRequest::get().uri("/orders/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Vec<models::Order> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 1);
        assert_eq!(body[0].item, "Burger");
        assert_eq!(body[0].cook_time, 10);
        assert_eq!(body[0].table_number, 1);
    }

    #[actix_web::test]
    async fn test_remove_order() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(pool.clone())
                .configure(route::config),
        )
        .await;

        // Insert test data
        sqlx::query!("INSERT INTO orders (id, table_number, item, cook_time) VALUES (?, ?, ?, ?)", 1, 2, "Sushi", 5)
            .execute(pool.get_ref().as_ref())
            .await
            .expect("Failed to insert order");

        let req = test::TestRequest::delete().uri("/orders/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let remaining_orders = sqlx::query!("SELECT * FROM orders WHERE id = ?", 1)
            .fetch_all(pool.get_ref().as_ref())
            .await
            .expect("Failed to fetch orders");
        assert!(remaining_orders.is_empty());
    }

    #[actix_web::test]
    async fn test_invalid_table() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(pool.clone())
                .configure(route::config),
        )
        .await;

        // Access unavailable Orders API
        let req = test::TestRequest::get().uri("/items").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_invalid_orders() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(pool.clone())
                .configure(route::config),
        )
        .await;

        // Access Incorrect Table API
        let req = test::TestRequest::get().uri("/orders/unavailable").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_cook_time() {
        let cook_time = utils::generate_random_cook_time();
        assert!(cook_time >= 5 && cook_time <= 15);
    }
}
