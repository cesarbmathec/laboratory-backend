use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TestType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cost: Decimal,
    pub sample_type: String, 
}

// --- ESTO ES LO QUE FALTABA ---
#[derive(Debug, Deserialize)]
pub struct CreateTestWithParameters {
    pub name: String,
    pub description: Option<String>,
    pub cost: Decimal,
    pub sample_type: String,
    pub parameters: Vec<CreateParameter>,
}

#[derive(Debug, Deserialize)]
pub struct CreateParameter {
    pub name: String,
    pub unit: Option<String>,
    pub reference_range: Option<String>,
    pub data_type: String,
}