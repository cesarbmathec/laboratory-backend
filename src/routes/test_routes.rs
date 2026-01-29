use actix_web::web;
use crate::handlers::test_handler::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/catalog")
            .route("/tests", web::post().to(create_test_catalog))
            .route("/tests", web::get().to(list_tests))
    );
}