use serde ::{Deserialize, Serialize};
use sqlx::FromRow; 

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Proveedor {
    pub id_proveedor: i32,
    pub nombre_empresa: String,
    pub telefono: Option<String>,
    pub contacto: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct NuevoProveedor {
    pub nombre_empresa: String,
    pub telefono: Option<String>,
    pub contacto: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarProveedor {
    pub nombre_empresa: Option<String>,
    pub telefono: Option<String>,
    pub contacto: Option<String>,
}