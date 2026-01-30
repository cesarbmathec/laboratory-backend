use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use actix_web::{HttpRequest, dev::Payload, error::ErrorUnauthorized, FromRequest};
use futures_util::future::{ready, Ready};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: Option<String>, // 'admin' o 'operator' según tu SQL
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiración
    pub role: String,
}

// Implementación del Extractor para Actix Web
impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_str) = auth_header.and_then(|h| h.to_str().ok()) {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ").to_string();
                let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
                
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::default(),
                );

                return match token_data {
                    Ok(data) => ready(Ok(data.claims)),
                    Err(_) => ready(Err(ErrorUnauthorized("Token inválido o expirado"))),
                };
            }
        }
        ready(Err(ErrorUnauthorized("Se requiere un token de autenticación")))
    }
}