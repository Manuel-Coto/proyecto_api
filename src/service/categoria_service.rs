use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::categoria::{ActualizarCategoria, Categoria, NuevaCategoria};
use crate::repository::categoria_repository::CategoriaRepository;

pub async fn obtener_todas(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Categoria>>, StatusCode> {
    let repo = CategoriaRepository::new(pool);

    match repo.obtener_categorias().await {
        Ok(categorias) => Ok(Json(categorias)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Categoria>, StatusCode> {
    let repo = CategoriaRepository::new(pool);

    match repo.obtener_categoria_por_id(id).await {
        Ok(categoria) => Ok(Json(categoria)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn crear_categoria(
    State(pool): State<PgPool>,
    Json(nueva_categoria): Json<NuevaCategoria>,
) -> Result<(StatusCode, Json<Categoria>), StatusCode> {
    let repo = CategoriaRepository::new(pool);

    match repo.crear_categoria(nueva_categoria).await {
        Ok(categoria) => Ok((StatusCode::CREATED, Json(categoria))),
        Err(e) => {
            eprintln!("Error al crear categoría: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn actualizar_categoria(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(cambios): Json<ActualizarCategoria>,
) -> Result<Json<Categoria>, StatusCode> {
    let repo = CategoriaRepository::new(pool);

    match repo.actualizar_categoria(id, cambios).await {
        Ok(categoria) => Ok(Json(categoria)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_categoria(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = CategoriaRepository::new(pool);

    match repo.eliminar_categoria(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}