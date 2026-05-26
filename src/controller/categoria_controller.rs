use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::service::categoria_service::{
    actualizar_categoria, crear_categoria, eliminar_categoria, obtener_por_id, obtener_todas,
};

pub fn categoria_router(pool: PgPool) -> Router {
    Router::new()
        .route("/categorias", get(obtener_todas))
        .route("/categorias/{id}", get(obtener_por_id))
        .route("/categorias", post(crear_categoria))
        .route("/categorias/{id}", put(actualizar_categoria))
        .route("/categorias/{id}", delete(eliminar_categoria))
        .with_state(pool)
}