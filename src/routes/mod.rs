// src/routes/mod.rs
use actix_web::web;
pub mod patient_routes;
pub mod test_routes;
pub mod order_routes;

pub fn main_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(patient_routes::config)
            .configure(test_routes::config)
            .configure(order_routes::config)
    );
}