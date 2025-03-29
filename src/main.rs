use actix_web::{App, HttpServer, web};
use sqlx::{MySql, Pool};
use std::{sync::Arc, thread};

mod api;
mod db;

/// AppState allows for me to share my pooled connection amongst all database routes.
/// This way we can have / maintain one connection, and go from there.
/// It must derive clone so we can clone the reference of it. It should be passed with a &*pool.clone()
#[derive(Clone)]
#[allow(dead_code)]
struct AppState {
    pool: Arc<Pool<MySql>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the number of cpu cores available that will determine the number of workers we create.
    let workers_available = thread::available_parallelism()
        .map(|p| p.get()) // Get the number of parallel threads available
        .unwrap_or(1); // Default to 1 if it fails

    // Initialize the database connection pool
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");
    let app_state = AppState {
        pool: Arc::new(pool),
    };

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_state.clone())) // add the pool to each of our route states as app data.
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 80))?
    .bind("127.0.0.1")?
    .workers(workers_available)
    .run()
    .await
}
