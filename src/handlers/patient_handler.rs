use crate::models::{
    patient::{CreatePatient, Patient},
    user::Claims,
};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use utoipa::ToSchema;

/// Datos para crear un nuevo paciente
#[derive(Debug, ToSchema)]
pub struct CreatePatientRequest {
    pub identifier: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: chrono::NaiveDate,
    pub gender: String,
}

/// Crear un nuevo paciente
/// 
/// Requiere autenticación JWT.
#[utoipa::path(
    post,
    path = "/api/patients",
    tag = "Patients",
    security(
        ("BearerAuth" = [])
    ),
    request_body = CreatePatientRequest,
    responses(
        (status = 201, description = "Paciente creado exitosamente"),
        (status = 400, description = "Error al crear paciente"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn create_patient(
    pool: web::Data<PgPool>,
    _user: Claims,
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
/// 
/// Requiere autenticación JWT.
#[utoipa::path(
    get,
    path = "/api/patients",
    tag = "Patients",
    security(
        ("BearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Lista de pacientes"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn get_patients(pool: web::Data<PgPool>, _user: Claims) -> impl Responder {
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
/// 
/// Requiere autenticación JWT.
#[utoipa::path(
    get,
    path = "/api/patients/{id}",
    tag = "Patients",
    security(
        ("BearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Paciente encontrado"),
        (status = 404, description = "Paciente no encontrado"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn get_patient_by_id(
    pool: web::Data<PgPool>,
    _user: Claims,
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
