use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    pub id_cliente: i32,
    pub nombre: String,
    pub apellido: String,
    pub correo_electronico: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoCliente {
    pub nombre: String,
    pub apellido: String,
    pub correo_electronico: String,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarCliente {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub correo_electronico: Option<String>,
}