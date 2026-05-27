use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::venta::{Venta, NuevaVenta, ActualizarVenta};
use crate::repository::venta_repository::VentaRepository;

pub async fn obtener_todas(
    State(pool): State<PgPool>
) -> Result<Json<Vec<Venta>>, StatusCode> {
    let repo = VentaRepository::new(pool);
    match repo.obtener_ventas().await {
        Ok(ventas) => Ok(Json(ventas)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Venta>, StatusCode> {
    let repo = VentaRepository::new(pool);
    match repo.obtener_venta_por_id(id).await {
        Ok(venta) => Ok(Json(venta)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn crear_venta(
    State(pool): State<PgPool>,
    Json(nueva_venta): Json<NuevaVenta>,
) -> Result<(StatusCode, Json<Venta>), StatusCode> {
    let repo = VentaRepository::new(pool);
    match repo.crear_venta(nueva_venta).await {
        Ok(venta) => Ok((StatusCode::CREATED, Json(venta))),
        Err(e) => {
            eprintln!("Error al crear venta: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn actualizar_venta(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(cambios): Json<ActualizarVenta>,
) -> Result<Json<Venta>, StatusCode> {
    let repo = VentaRepository::new(pool);
    match repo.actualizar_venta(id, cambios).await {
        Ok(venta) => Ok(Json(venta)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_venta(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = VentaRepository::new(pool);
    match repo.eliminar_venta(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}