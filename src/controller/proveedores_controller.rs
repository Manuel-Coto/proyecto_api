use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::service::proveedores_service::{
    actualizar_proveedor, crear_proveedor, eliminar_proveedor, obtener_por_id, obtener_todos,
};  

pub fn proveedores_router(pool: PgPool) -> Router {
    Router::new()
        .route("/proveedores", get(obtener_todos))
        .route("/proveedores/{id}", get(obtener_por_id))
        .route("/proveedores", post(crear_proveedor))
        .route("/proveedores/{id}", put(actualizar_proveedor))
        .route("/proveedores/{id}", delete(eliminar_proveedor))
        .with_state(pool)
}