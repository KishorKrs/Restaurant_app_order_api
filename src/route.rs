use actix_web::web;
use crate::api;

// Add route to the Application
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/orders")
        .route("", web::post().to(api::add_order))                              // POST Request
        .route("", web::get().to(api::get_order))                               // GET Request: with id
        .route("/{table_number}", web::get().to(api::get_orders_for_table))     // GET Request: for table
        .route("/{order_id}", web::delete().to(api::remove_order))              // DELERE Request
        .route("*", web::to(api::handle_404))                                        // Handle Non existance uri inside orders
    );

    cfg.route("*", web::to(api::handle_404));                                        // Handle all non-existent URLs
}
