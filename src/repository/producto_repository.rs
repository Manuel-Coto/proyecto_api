use sqlx::PgPool;
use crate::models::producto::{Producto, NuevoProducto, ActualizarProducto};

pub struct ProductoRepository {
    pool: PgPool,
}

impl ProductoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_producto(&self) -> sqlx::Result<Vec<Producto>> {
        let productos = sqlx::query_as::<_, Producto>(
            "SELECT id_producto, nombre, precio::float8, stock, id_categoria, id_proveedor FROM Productos"
        ).fetch_all(&self.pool).await?;
        Ok(productos)
    }

    pub async fn crear_producto(&self, nuevo: NuevoProducto) -> sqlx::Result<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"INSERT INTO Productos (nombre, precio, stock, id_categoria, id_proveedor)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING id_producto, nombre, precio::float8, stock, id_categoria, id_proveedor"#
        )
            .bind(nuevo.nombre)
            .bind(nuevo.precio)
            .bind(nuevo.stock)
            .bind(nuevo.categoria)
            .bind(nuevo.proveedor)
            .fetch_one(&self.pool)
            .await?;
        Ok(producto)
    }

    pub async fn actualizar_producto(&self, id: i32, cambios: ActualizarProducto) -> sqlx::Result<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"UPDATE Productos
               SET nombre       = COALESCE($1, nombre),
                   precio       = COALESCE($2, precio),
                   stock        = COALESCE($3, stock),
                   id_categoria = COALESCE($4, id_categoria),
                   id_proveedor = COALESCE($5, id_proveedor)
               WHERE id_producto = $6
               RETURNING id_producto, nombre, precio::float8, stock, id_categoria, id_proveedor"#
        )
            .bind(cambios.nombre)
            .bind(cambios.precio)
            .bind(cambios.stock)
            .bind(cambios.categoria)
            .bind(cambios.proveedor)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(producto)
    }

    pub async fn eliminar_producto(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Productos WHERE id_producto = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn obtener_producto_por_id(&self, id: i32) -> sqlx::Result<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            "SELECT id_producto, nombre, precio::float8, stock, id_categoria, id_proveedor FROM Productos WHERE id_producto = $1"
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(producto)
    }
}