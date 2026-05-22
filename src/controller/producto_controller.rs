use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::producto_service::{
    obtener_todos,
    obtener_por_id,
    crear_producto,
    actualizar_producto,
    eliminar_producto,
};

pub fn producto_router(pool: PgPool) -> Router {
    Router::new()
        .route("/productos", get(obtener_todos))
        .route("/productos/{id}", get(obtener_por_id))
        .route("/productos", post(crear_producto))
        .route("/productos/{id}", put(actualizar_producto))
        .route("/productos/{id}", delete(eliminar_producto))
        .with_state(pool)
}