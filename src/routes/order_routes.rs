use actix_web::web;
use crate::handlers::order_handler::{create_order, get_orders};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("", web::post().to(create_order))
            .route("", web::get().to(get_orders))   // Listar usando OrderSummary
    );
}