// src/openapi.rs
use utoipa::OpenApi;
use crate::models::user::{AuthRequest, AuthResponse};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Laboratory API",
        description = "API para gestión de laboratorio clínico",
        version = "1.0.0",
        contact(
            name = "Support",
            email = "support@laboratory.com"
        )
    ),
    servers(
        (url = "http://127.0.0.1:8080")
    ),
    components(
        schemas(
            AuthRequest,
            AuthResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Endpoints de autenticación"),
        (name = "Patients", description = "Gestión de pacientes"),
        (name = "Orders", description = "Gestión de órdenes"),
        (name = "Tests", description = "Gestión de tipos de exámenes"),
        (name = "Results", description = "Gestión de resultados")
    )
)]
pub struct ApiDoc;
