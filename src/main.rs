mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

//cambiar por los controllers que tengan
use controller::producto_controller::producto_router;
use controller::categoria_controller::categoria_router;
use controller::proveedores_controller::proveedores_router;
use controller::venta_controller::venta_router;
use controller::cliente_controller::cliente_router;
use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}



fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    let router1 = producto_router(pool.clone());
    let router2 = categoria_router(pool.clone());
    let router3 = proveedores_router(pool.clone());
    let router4 = venta_router(pool.clone());
    let router5 = cliente_router(pool.clone());

    router1.merge(router2).merge(router3).merge(router4).merge(router5)
}