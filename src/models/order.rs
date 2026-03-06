use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub patient_id: i32,
    pub test_ids: Vec<i32>, // IDs de los exámenes que se va a realizar
    pub created_by: i32,    // ID del usuario/operador
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderSummary {
    pub id: i32,
    pub patient_id: i32,
    pub total_amount: Decimal,
    pub payment_status: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderDetail {
    pub id: i32,
    pub patient_name: String,
    pub patient_identifier: String,
    pub total_amount: rust_decimal::Decimal,
    pub payment_status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderTest {
    pub test_name: String,
    pub test_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderFullResponse {
    pub order: OrderDetail,
    pub tests: Vec<OrderTest>,
}