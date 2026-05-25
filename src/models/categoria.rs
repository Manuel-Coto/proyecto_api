use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Categoria {
    #[sqlx(rename = "id_categoria")]
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaCategoria {
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarCategoria {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
}