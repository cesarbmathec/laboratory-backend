use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub id: i32,
    pub identifier: String, // Cédula o DNI
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub gender: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CreatePatient {
    pub identifier: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub gender: String,
}