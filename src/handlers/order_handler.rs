use crate::models::order::{CreateOrder, OrderSummary};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

// Crear una nueva orden
pub async fn create_order(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateOrder>,
) -> impl Responder {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Error de transacción"),
    };

    // Calcular el total con Type Override
    let total = match sqlx::query!(
        r#"SELECT SUM(cost) as "total!: rust_decimal::Decimal" FROM test_types WHERE id = ANY($1)"#,
        &payload.test_ids
    )
    .fetch_one(&mut *tx)
    .await
    {
        Ok(rec) => rec.total,
        Err(e) => {
            eprintln!("Error sumando costos: {:?}", e);
            return HttpResponse::BadRequest().body("Uno o más IDs de examen son inválidos");
        }
    };

    // Crear la Orden
    let order_id = match sqlx::query!(
        r#"INSERT INTO orders (patient_id, total_amount, created_by, payment_status) 
       VALUES ($1, $2, $3, 'PAID') RETURNING id"#,
        payload.patient_id,
        total as _, // El "as _" le dice a la macro que use el tipo de la variable 'total'
        payload.created_by
    )
    .fetch_one(&mut *tx)
    .await
    {
        Ok(rec) => rec.id,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    // Crear los registros de resultados vacíos para cada parámetro de cada examen
    // Esto es para que el bioanalista ya tenga la "plantilla" lista para llenar
    let res = sqlx::query!(
        r#"
        INSERT INTO results (order_id, parameter_id)
        SELECT $1, tp.id 
        FROM test_parameters tp
        WHERE tp.test_type_id = ANY($2)
        "#,
        order_id,
        &payload.test_ids
    )
    .execute(&mut *tx)
    .await;

    if res.is_err() || tx.commit().await.is_err() {
        return HttpResponse::InternalServerError().body("Error al finalizar la orden");
    }

    HttpResponse::Created().json(format!(
        "Orden #{} creada por un total de {}",
        order_id, total
    ))
}

// Obtener todas las órdenes
pub async fn get_orders(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        OrderSummary,
        r#"
        SELECT 
            id, 
            patient_id as "patient_id!", 
            total_amount as "total_amount: rust_decimal::Decimal", 
            payment_status as "payment_status!"
        FROM orders 
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("Error al obtener órdenes: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}