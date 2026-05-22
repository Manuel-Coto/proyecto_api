use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Producto{
    #[sqlx(rename = "id_producto")]
    pub id: i32,
    pub nombre: String,
    pub precio: f64,
    pub stock: i32,
    #[sqlx(rename ="id_categoria")]
    pub categoria: i32,
    #[sqlx(rename = "id_proveedor")]
    pub proveedor: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoProducto{
    pub nombre: String,
    pub precio: f64,
    pub stock: i32,
    pub categoria: i32,
    pub proveedor: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarProducto {
    pub nombre: Option<String>,
    pub precio: Option<f64>,
    pub stock: Option<i32>,
    pub categoria: Option<i32>,
    pub proveedor: Option<i32>,
}

