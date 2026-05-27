use sqlx::PgPool;
use crate::models::venta::{Venta, NuevaVenta, ActualizarVenta};

pub struct VentaRepository {
    pool: PgPool,
}

impl VentaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_ventas(&self) -> sqlx::Result<Vec<Venta>> {
        let ventas = sqlx::query_as::<_, Venta>(
            "SELECT id_venta, id_producto, cantidad, total::float8, fecha_venta FROM Ventas"
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(ventas)
    }

    pub async fn obtener_venta_por_id(&self, id: i32) -> sqlx::Result<Venta> {
        let venta = sqlx::query_as::<_, Venta>(
            "SELECT id_venta, id_producto, cantidad, total::float8, fecha_venta FROM Ventas WHERE id_venta = $1"
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(venta)
    }

    pub async fn crear_venta(&self, nueva: NuevaVenta) -> sqlx::Result<Venta> {
        let venta = sqlx::query_as::<_, Venta>(
            r#"INSERT INTO Ventas (id_producto, cantidad, total)
               VALUES ($1, $2, $3)
               RETURNING id_venta, id_producto, cantidad, total::float8, fecha_venta"#
        )
            .bind(nueva.producto)
            .bind(nueva.cantidad)
            .bind(nueva.total)
            .fetch_one(&self.pool)
            .await?;
        Ok(venta)
    }

    pub async fn actualizar_venta(&self, id: i32, cambios: ActualizarVenta) -> sqlx::Result<Venta> {
        let venta = sqlx::query_as::<_, Venta>(
            r#"UPDATE Ventas
               SET cantidad = COALESCE($1, cantidad),
                   total    = COALESCE($2, total)
               WHERE id_venta = $3
               RETURNING id_venta, id_producto, cantidad, total::float8, fecha_venta"#
        )
            .bind(cambios.cantidad)
            .bind(cambios.total)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(venta)
    }

    pub async fn eliminar_venta(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Ventas WHERE id_venta = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}