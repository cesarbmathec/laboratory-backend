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