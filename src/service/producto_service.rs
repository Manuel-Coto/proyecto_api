use axum::{extract::{Path, State}, http::StatusCode, Json };
use sqlx::PgPool;
use crate::models::producto::{Producto, NuevoProducto, ActualizarProducto};
use crate::repository::producto_repository::ProductoRepository;

// 1. OBTENER TODOS LOS PRODUCTOS (GET /productos)
pub async fn obtener_todos(
    State(pool): State<PgPool>
) -> Result<Json<Vec<Producto>>, StatusCode> {
    let repo = ProductoRepository::new(pool);
    match repo.obtener_producto().await {
        Ok(productos) => Ok(Json(productos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 2. OBTENER UN PRODUCTO POR ID
pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Producto>, StatusCode> {
    let repo = ProductoRepository::new(pool);
    match repo.obtener_producto_por_id(id).await {
        Ok(producto) => Ok(Json(producto)),
        Err(_) => Err(StatusCode::NOT_FOUND), // 404 si el producto no existe
    }
}

// 3. CREAR UN PRODUCTO )
pub async fn crear_producto(
    State(pool): State<PgPool>,
    Json(nuevo_producto): Json<NuevoProducto>,
) -> Result<(StatusCode, Json<Producto>), StatusCode> {
    let repo = ProductoRepository::new(pool);
    match repo.crear_producto(nuevo_producto).await {
        Ok(producto) => Ok((StatusCode::CREATED, Json(producto))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

// 4. ACTUALIZAR UN PRODUCTO
pub async fn actualizar_producto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(cambios): Json<ActualizarProducto>,
) -> Result<Json<Producto>, StatusCode> {
    let repo = ProductoRepository::new(pool);
    match repo.actualizar_producto(id, cambios).await {
        Ok(producto) => Ok(Json(producto)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 5. ELIMINAR UN PRODUCTO
pub async fn eliminar_producto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = ProductoRepository::new(pool);
    match repo.eliminar_producto(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}