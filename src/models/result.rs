use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct ResultDetail {
    pub id: i32,                // ID de la tabla 'results'
    pub parameter_name: String,  // Ej: "Hemoglobina"
    pub unit: Option<String>,    // Ej: "g/dL"
    pub reference_range: Option<String>, // Ej: "12-16"
    pub test_value: Option<String>,      // El valor que el bioanalista ingresará
    pub technician_name: Option<String>, // Nombre del técnico que validó el resultado
}

#[derive(Debug, Deserialize)]
pub struct UpdateResultPayload {
    pub test_value: String,
    pub is_abnormal: bool,
}