use crate::models::patient::{CreatePatient, Patient};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn create_patient(
    pool: web::Data<PgPool>,
    payload: web::Json<CreatePatient>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO patients (identifier, first_name, last_name, birth_date, gender)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        payload.identifier,
        payload.first_name,
        payload.last_name,
        payload.birth_date,
        payload.gender
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            HttpResponse::Created().json(format!("Paciente creado con ID: {}", record.id))
        }
        Err(e) => {
            eprintln!("Error al crear paciente: {}", e);
            HttpResponse::InternalServerError().body("Error al guardar el paciente")
        }
    }
}

// Obtener todos los pacientes
pub async fn get_patients(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Patient,
        "SELECT id, identifier, first_name, last_name, birth_date, gender FROM patients ORDER BY id DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(e) => {
            eprintln!("Error al obtener pacientes: {}", e);
            HttpResponse::InternalServerError().body("Error al consultar la base de datos")
        }
    }
}

// Obtener un paciente por ID
pub async fn get_patient_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>, // Extrae el ID de la URL
) -> impl Responder {
    let patient_id = path.into_inner();
    let result = sqlx::query_as!(
        Patient,
        "SELECT id, identifier, first_name, last_name, birth_date, gender FROM patients WHERE id = $1",
        patient_id
    )
    .fetch_optional(pool.get_ref()) // Puede o no existir
    .await;

    match result {
        Ok(Some(patient)) => HttpResponse::Ok().json(patient),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No existe el paciente con ID: {}", patient_id))
        }
        Err(e) => {
            eprintln!("Error al obtener paciente: {}", e);
            HttpResponse::InternalServerError().body("Error al consultar la base de datos")
        }
    }
}
