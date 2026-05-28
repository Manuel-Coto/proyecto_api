use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::venta_service::{
    obtener_todas,
    obtener_por_id,
    crear_venta,
    actualizar_venta,
    eliminar_venta,
};

pub fn venta_router(pool: PgPool) -> Router {
    Router::new()
        .route("/ventas", get(obtener_todas))
        .route("/ventas/{id}", get(obtener_por_id))
        .route("/ventas", post(crear_venta))
        .route("/ventas/{id}", put(actualizar_venta))
        .route("/ventas/{id}", delete(eliminar_venta))
        .with_state(pool)
}