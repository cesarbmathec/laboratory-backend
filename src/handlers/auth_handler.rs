use crate::models::user::{AuthRequest, AuthResponse, Claims, User};
use actix_web::{HttpResponse, Responder, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;
use std::env;

pub async fn login(pool: web::Data<PgPool>, payload: web::Json<AuthRequest>) -> impl Responder {
    // Buscar usuario en la DB
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, password_hash, role::text as "role!" FROM users WHERE username = $1"#,
        payload.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    if let Ok(Some(user_rec)) = user {
        // Verificar contraseña
        if verify(&payload.password, &user_rec.password_hash).unwrap_or(false) {
            // Generar JWT
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: user_rec.id.to_string(),
                exp: expiration as usize,
                role: user_rec.role.unwrap_or_else(|| "operator".to_string()),
            };

            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .unwrap();

            return HttpResponse::Ok().json(AuthResponse {
                token,
                username: user_rec.username,
                role: claims.role,
            });
        }
    }

    HttpResponse::Unauthorized().body("Credenciales inválidas")
}

pub async fn register(pool: web::Data<PgPool>, payload: web::Json<AuthRequest>) -> impl Responder {
    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Error al procesar contraseña"),
    };

    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, 'operator'::user_role) RETURNING id",
        payload.username,
        hashed_password
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(rec) => HttpResponse::Created().json(format!(
            "Usuario {} creado con ID {}",
            payload.username, rec.id
        )),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}
