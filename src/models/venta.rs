use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Representa una venta registrada en la base de datos.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Venta {
    #[sqlx(rename = "id_venta")]
    pub id: i32,

    #[sqlx(rename = "id_producto")]
    pub producto: i32,

    pub cantidad: i32,

    pub total: f64,

    pub fecha_venta: chrono::NaiveDateTime,
}

/// Datos requeridos para registrar una nueva venta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaVenta {
    pub producto: i32,
    pub cantidad: i32,
    pub total: f64,
}

/// Campos opcionales para actualizar una venta existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarVenta {
    pub cantidad: Option<i32>,
    pub total: Option<f64>,
}