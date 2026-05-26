use sqlx::PgPool;

use crate::models::categoria::{ActualizarCategoria, Categoria, NuevaCategoria};

pub struct CategoriaRepository {
    pool: PgPool,
}

impl CategoriaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Obtiene todas las categorías registradas en la base de datos.
    pub async fn obtener_categorias(&self) -> sqlx::Result<Vec<Categoria>> {
        let categorias = sqlx::query_as::<_, Categoria>(
            "SELECT id_categoria, nombre, descripcion FROM Categorias ORDER BY id_categoria",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categorias)
    }

    pub async fn obtener_categoria_por_id(&self, id: i32) -> sqlx::Result<Categoria> {
        let categoria = sqlx::query_as::<_, Categoria>(
            "SELECT id_categoria, nombre, descripcion FROM Categorias WHERE id_categoria = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(categoria)
    }

    // Inserta una nueva categoría en la base de datos.
    pub async fn crear_categoria(&self, nueva: NuevaCategoria) -> sqlx::Result<Categoria> {
        let categoria = sqlx::query_as::<_, Categoria>(
            r#"
            INSERT INTO Categorias (nombre, descripcion)
            VALUES ($1, $2)
            RETURNING id_categoria, nombre, descripcion
            "#,
        )
        .bind(nueva.nombre)
        .bind(nueva.descripcion)
        .fetch_one(&self.pool)
        .await?;

        Ok(categoria)
    }

    // Actualiza una categoría existente.
    // COALESCE permite conservar el valor actual cuando el campo viene como null.
    pub async fn actualizar_categoria(
        &self,
        id: i32,
        cambios: ActualizarCategoria,
    ) -> sqlx::Result<Categoria> {
        let categoria = sqlx::query_as::<_, Categoria>(
            r#"
            UPDATE Categorias
            SET nombre = COALESCE($1, nombre),
                descripcion = COALESCE($2, descripcion)
            WHERE id_categoria = $3
            RETURNING id_categoria, nombre, descripcion
            "#,
        )
        .bind(cambios.nombre)
        .bind(cambios.descripcion)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(categoria)
    }

    // Elimina una categoría por su id.
    pub async fn eliminar_categoria(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Categorias WHERE id_categoria = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
