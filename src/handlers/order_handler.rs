use crate::models::{order::{CreateOrder, OrderSummary, OrderDetail, OrderTest, OrderFullResponse}, user::Claims};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use utoipa::ToSchema;

/// Datos para crear una nueva orden
#[derive(Debug, ToSchema)]
pub struct CreateOrderRequest {
    pub patient_id: i32,
    pub test_ids: Vec<i32>,
}

// Crear una nueva orden
/// 
/// Requiere autenticación JWT. Crea una nueva orden de exámenes para un paciente.
#[utoipa::path(
    post,
    path = "/api/orders",
    tag = "Orders",
    security(
        ("BearerAuth" = [])
    ),
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "Orden creada exitosamente"),
        (status = 400, description = "Error en los datos de la orden"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn create_order(
    pool: web::Data<PgPool>,
    user: Claims,
    payload: web::Json<CreateOrder>,
) -> impl Responder {
    let creator_id: i32 = user.sub.parse().unwrap_or(0);
    
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
        creator_id
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
/// 
/// Requiere autenticación JWT.
#[utoipa::path(
    get,
    path = "/api/orders",
    tag = "Orders",
    security(
        ("BearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Lista de órdenes"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn get_orders(pool: web::Data<PgPool>, _user: Claims) -> impl Responder {
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

/// Obtener una orden por ID
/// 
/// Requiere autenticación JWT.
#[utoipa::path(
    get,
    path = "/api/orders/{id}",
    tag = "Orders",
    security(
        ("BearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Orden encontrada"),
        (status = 404, description = "Orden no encontrada"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn get_order_by_id(
    pool: web::Data<PgPool>,
    _user: Claims,
    path: web::Path<i32>,
) -> impl Responder {
    let order_id = path.into_inner();

    // 1. Obtener datos de la orden y el paciente
    let order_detail = sqlx::query_as!(
        OrderDetail,
        r#"
        SELECT 
            o.id, 
            (p.first_name || ' ' || p.last_name) as "patient_name!", 
            p.identifier as "patient_identifier!",
            o.total_amount as "total_amount: rust_decimal::Decimal", 
            o.payment_status as "payment_status!",
            o.created_at
        FROM orders o
        JOIN patients p ON o.patient_id = p.id
        WHERE o.id = $1
        "#,
        order_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match order_detail {
        Ok(Some(order)) => {
            // 2. Obtener los nombres de los exámenes de esta orden
            // Nota: Buscamos en 'results' agrupando por 'test_type_id'
            let tests = sqlx::query_as!(
                OrderTest,
                r#"
                SELECT DISTINCT tt.name as "test_name!", tt.description as "test_description"
                FROM results r
                JOIN test_parameters tp ON r.parameter_id = tp.id
                JOIN test_types tt ON tp.test_type_id = tt.id
                WHERE r.order_id = $1
                "#,
                order_id
            )
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_default();

            HttpResponse::Ok().json(OrderFullResponse { order, tests })
        }
        Ok(None) => HttpResponse::NotFound().body("Orden no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
