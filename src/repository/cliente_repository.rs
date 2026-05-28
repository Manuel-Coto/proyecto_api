use sqlx::PgPool;
use crate::models::cliente::{Cliente, NuevoCliente, ActualizarCliente};

pub struct ClienteRepository {
    pool: PgPool,
}

impl ClienteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Obtener todos los clientes
    pub async fn obtener_clientes(&self) -> sqlx::Result<Vec<Cliente>> {
        let clientes = sqlx::query_as::<_, Cliente>(
            "SELECT id_cliente, nombre, apellido, correo_electronico FROM clientes ORDER BY id_cliente",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clientes)
    }

    // Obtener cliente por ID
    pub async fn obtener_cliente_por_id(&self, id: i32) -> sqlx::Result<Cliente> {
        let cliente = sqlx::query_as::<_, Cliente>(
            "SELECT id_cliente, nombre, apellido, correo_electronico FROM clientes WHERE id_cliente = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(cliente)
    }

    // Insertar un nuevo cliente
    pub async fn crear_cliente(&self, nueva: NuevoCliente) -> sqlx::Result<Cliente> {
        let cliente = sqlx::query_as::<_, Cliente>(
            r#"
            INSERT INTO clientes (nombre, apellido, correo_electronico)
            VALUES ($1, $2, $3)
            RETURNING id_cliente, nombre, apellido, correo_electronico
            "#,
        )
        .bind(nueva.nombre)
        .bind(nueva.apellido)
        .bind(nueva.correo_electronico)
        .fetch_one(&self.pool)
        .await?;

        Ok(cliente)
    }

    // Actualizar cliente usando COALESCE
    pub async fn actualizar_cliente(&self, id: i32, cambios: ActualizarCliente) -> sqlx::Result<Cliente> {
        let cliente = sqlx::query_as::<_, Cliente>(
            r#"
            UPDATE clientes
            SET nombre = COALESCE($1, nombre),
                apellido = COALESCE($2, apellido),
                correo_electronico = COALESCE($3, correo_electronico)
            WHERE id_cliente = $4
            RETURNING id_cliente, nombre, apellido, correo_electronico
            "#,
        )
        .bind(cambios.nombre)
        .bind(cambios.apellido)
        .bind(cambios.correo_electronico)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(cliente)
    }

    // Eliminar un cliente por ID
    pub async fn eliminar_cliente(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM clientes WHERE id_cliente = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}