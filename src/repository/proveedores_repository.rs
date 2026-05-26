use sqlx::PgPool;
use crate::models::proveedores::{ActualizarProveedor, NuevoProveedor, Proveedor};

pub struct ProveedoresRepository {
    pool: PgPool,
}

impl ProveedoresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_proveedores(&self) -> sqlx::Result<Vec<Proveedor>> {
        let proveedores = sqlx::query_as::<_, Proveedor>(
            "SELECT id_proveedor, nombre_empresa, telefono, contacto FROM Proveedores ORDER BY id_proveedor",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(proveedores)
    }

    pub async fn obtener_proveedor_por_id(&self, id: i32) -> sqlx::Result<Proveedor> {
        let proveedor = sqlx::query_as::<_, Proveedor>(
            "SELECT id_proveedor, nombre_empresa, telefono, contacto FROM Proveedores WHERE id_proveedor = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(proveedor)
    }

    pub async fn crear_proveedor(&self, nuevo: NuevoProveedor) -> sqlx::Result<Proveedor> {
        let proveedor = sqlx::query_as::<_, Proveedor>(
            r#"
            INSERT INTO Proveedores (nombre_empresa, telefono, contacto)
            VALUES ($1, $2, $3)
            RETURNING id_proveedor, nombre_empresa, telefono, contacto
            "#,
        )
        .bind(nuevo.nombre_empresa)
        .bind(nuevo.telefono)
        .bind(nuevo.contacto)
        .fetch_one(&self.pool)
        .await?;

        Ok(proveedor)
    }

    pub async fn actualizar_proveedor(
        &self,
        id: i32,
        cambios: ActualizarProveedor,
    ) -> sqlx::Result<Proveedor> {
        let proveedor = sqlx::query_as::<_, Proveedor>(
            r#"
            UPDATE Proveedores
            SET nombre_empresa = COALESCE($1, nombre_empresa),
                telefono = COALESCE($2, telefono),
                contacto = COALESCE($3, contacto)
            WHERE id_proveedor = $4
            RETURNING id_proveedor, nombre_empresa, telefono, contacto
            "#,
        )
        .bind(cambios.nombre_empresa)
        .bind(cambios.telefono)
        .bind(cambios.contacto)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(proveedor)
    }

    pub async fn eliminar_proveedor(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Proveedores WHERE id_proveedor = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}