use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::cliente::{ActualizarCliente, Cliente, NuevoCliente};
use crate::repository::cliente_repository::ClienteRepository;

pub async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Cliente>>, StatusCode> {
    let repo = ClienteRepository::new(pool);

    match repo.obtener_clientes().await {
        Ok(clientes) => Ok(Json(clientes)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ¡Aquí corregimos el detalle para que use únicamente ClienteRepository!
pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Cliente>, StatusCode> {
    let repo = ClienteRepository::new(pool);

    match repo.obtener_cliente_por_id(id).await {
        Ok(cliente) => Ok(Json(cliente)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn crear_cliente(
    State(pool): State<PgPool>,
    Json(nuevo_cliente): Json<NuevoCliente>,
) -> Result<(StatusCode, Json<Cliente>), StatusCode> {
    let repo = ClienteRepository::new(pool);

    match repo.crear_cliente(nuevo_cliente).await {
        Ok(cliente) => Ok((StatusCode::CREATED, Json(cliente))),
        Err(e) => {
            eprintln!("Error al crear cliente: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn actualizar_cliente(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(cambios): Json<ActualizarCliente>,
) -> Result<Json<Cliente>, StatusCode> {
    let repo = ClienteRepository::new(pool);

    match repo.actualizar_cliente(id, cambios).await {
        Ok(cliente) => Ok(Json(cliente)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_cliente(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = ClienteRepository::new(pool);

    match repo.eliminar_cliente(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}