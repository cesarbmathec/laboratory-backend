use actix_web::web;
use crate::handlers::patient_handler::{get_patient_by_id, get_patients, create_patient};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patients")
            .route("", web::post().to(create_patient))      // Crear
            .route("", web::get().to(get_patients))        // Listar todos
            .route("/{id}", web::get().to(get_patient_by_id)) // Buscar uno
    );
}