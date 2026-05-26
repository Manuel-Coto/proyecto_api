use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::proveedores::{ActualizarProveedor, NuevoProveedor, Proveedor};
use crate::repository::proveedores_repository::ProveedoresRepository;   

pub async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Proveedor>>, StatusCode> {
    let repo = ProveedoresRepository::new(pool);

    match repo.obtener_proveedores().await {
        Ok(proveedores) => Ok(Json(proveedores)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Proveedor>, StatusCode> {
    let repo = ProveedoresRepository::new(pool);

    match repo.obtener_proveedor_por_id(id).await {
        Ok(proveedor) => Ok(Json(proveedor)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn crear_proveedor(
    State(pool): State<PgPool>,
    Json(nuevo_proveedor): Json<NuevoProveedor>,
) -> Result<(StatusCode, Json<Proveedor>), StatusCode> {
    let repo = ProveedoresRepository::new(pool);

    match repo.crear_proveedor(nuevo_proveedor).await {
        Ok(proveedor) => Ok((StatusCode::CREATED, Json(proveedor))),
        Err(e) => {
            eprintln!("Error al crear proveedor: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn actualizar_proveedor(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(cambios): Json<ActualizarProveedor>,
) -> Result<Json<Proveedor>, StatusCode> {
    let repo = ProveedoresRepository::new(pool);

    match repo.actualizar_proveedor(id, cambios).await {
        Ok(proveedor) => Ok(Json(proveedor)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_proveedor(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = ProveedoresRepository::new(pool);

    match repo.eliminar_proveedor(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}   