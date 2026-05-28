use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::service::cliente_service::{
    actualizar_cliente, crear_cliente, eliminar_cliente, obtener_por_id, obtener_todos,
};

pub fn cliente_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/clientes", get(obtener_todos))
        .route("/api/clientes/{id}", get(obtener_por_id))
        .route("/api/clientes", post(crear_cliente))
        .route("/api/clientes/{id}", put(actualizar_cliente))
        .route("/api/clientes/{id}", delete(eliminar_cliente))
        .with_state(pool)
}