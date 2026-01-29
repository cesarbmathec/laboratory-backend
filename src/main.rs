use actix_web::{App, HttpServer, Responder, http::header, middleware::Logger, web};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use env_logger::Env;
use actix_cors::Cors;

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar el logger con nivel "info" por defecto
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Cargar variables de entorno (.env)
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurado en el archivo .env");

    // Crear el pool de conexión a la base de datos
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    println!("🚀 Servidor corriendo en http://127.0.0.1:8080");

    // Iniciar el servidor
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Cambia esto por la URL de tu frontend
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            // Formateo de log personalizado que muestre el contenido de la petición y el tiempo de respuesta
            .wrap(Logger::new("%a %r %s %b %{Referer}i %T"))
            .app_data(web::Data::new(pool.clone())) // Compartir la conexión con los handlers
            .route("/", web::get().to(health_check))
            .configure(routes::main_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    "Servidor de Laboratorio funcionando correctamente"
}