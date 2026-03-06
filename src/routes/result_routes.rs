use actix_web::web;
use crate::handlers::result_handler::{get_results_by_order, update_result_value};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/results")
            .route("/order/{order_id}", web::get().to(get_results_by_order))
            .route("/{result_id}", web::patch().to(update_result_value))
    );
}