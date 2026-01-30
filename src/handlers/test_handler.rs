use crate::models::{
    test::{CreateTestWithParameters, TestType},
    user::Claims,
};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn create_test_catalog(
    pool: web::Data<PgPool>,
    _user: Claims,
    payload: web::Json<CreateTestWithParameters>,
) -> impl Responder {
    // Iniciamos una transacción
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Error al iniciar transacción"),
    };

    // Insertar el tipo de examen
    let test_id = match sqlx::query!(
        r#"
    INSERT INTO test_types (name, description, cost, sample_type) 
    VALUES ($1, $2, $3, $4::text::sample_category) 
    RETURNING id
    "#,
        payload.name,
        payload.description,
        payload.cost as _,
        payload.sample_type // Se envía como String y Postgres lo convierte
    )
    .fetch_one(&mut *tx)
    .await
    {
        Ok(rec) => rec.id,
        Err(e) => return HttpResponse::BadRequest().body(format!("Error al crear tipo: {}", e)),
    };

    // Insertar cada parámetro asociado
    for param in &payload.parameters {
        if let Err(e) =
            sqlx::query!(
            "INSERT INTO test_parameters (test_type_id, name, unit, reference_range, data_type) 
             VALUES ($1, $2, $3, $4, $5)",
            test_id, param.name, param.unit, param.reference_range, param.data_type
        )
            .execute(&mut *tx)
            .await
        {
            return HttpResponse::BadRequest()
                .body(format!("Error en parámetro {}: {}", param.name, e));
        }
    }

    // Confirmar cambios
    if tx.commit().await.is_err() {
        return HttpResponse::InternalServerError().body("Error al confirmar transacción");
    }

    HttpResponse::Created().json(format!("Examen '{}' creado exitosamente", payload.name))
}

pub async fn list_tests(pool: web::Data<PgPool>, _user: Claims) -> impl Responder {
    let tests = sqlx::query_as!(
        TestType,
        r#"
        SELECT 
            id, 
            name, 
            description, 
            cost as "cost: rust_decimal::Decimal", 
            sample_type::text as "sample_type!"
        FROM test_types
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match tests {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            eprintln!("Error al listar exámenes: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
