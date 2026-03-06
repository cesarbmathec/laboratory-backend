use crate::models::{
    result::{ResultDetail, UpdateResultPayload},
    user::Claims,
};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

// 1. Obtener todos los parámetros de una orden para llenar resultados
/// 
/// Requiere autenticación JWT.
#[utoipa::path(
    get,
    path = "/api/results/{order_id}",
    tag = "Results",
    security(
        ("BearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Resultados de la orden"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn get_results_by_order(
    pool: web::Data<PgPool>,
    _user: Claims,
    path: web::Path<i32>,
) -> impl Responder {
    let order_id = path.into_inner();

    let results = sqlx::query_as!(
        ResultDetail,
        r#"
        SELECT 
            r.id, 
            tp.name as "parameter_name!", 
            tp.unit, 
            tp.reference_range, 
            r.test_value,
            u.username as "technician_name" -- Obtenemos el nombre del bioanalista
        FROM results r
        JOIN test_parameters tp ON r.parameter_id = tp.id
        LEFT JOIN users u ON r.technician_id = u.id -- Unimos con la tabla de usuarios
        WHERE r.order_id = $1
        "#,
        order_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match results {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// 2. Actualizar un resultado individual
/// 
/// Requiere autenticación JWT. Actualiza el valor de un resultado y lo marca como validado.
#[utoipa::path(
    put,
    path = "/api/results/{result_id}",
    tag = "Results",
    security(
        ("BearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Resultado actualizado"),
        (status = 401, description = "No autorizado"),
        (status = 500, description = "Error al actualizar resultado")
    )
)]
pub async fn update_result_value(
    pool: web::Data<PgPool>,
    user: Claims,
    path: web::Path<i32>, // ID del resultado
    payload: web::Json<UpdateResultPayload>,
) -> impl Responder {
    let result_id = path.into_inner();
    let technician_id: i32 = user.sub.parse().unwrap_or(0);

    let res = sqlx::query!(
        r#"
        UPDATE results 
        SET test_value = $1, is_abnormal = $2, technician_id = $3, validated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        "#,
        payload.test_value,
        payload.is_abnormal,
        technician_id,
        result_id
    )
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().body("Resultado actualizado"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
