use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};
use std::{sync::Arc, thread};

use crate::api::v1::comment::*;

mod api;
mod db;

// Define shared application state
#[derive(Clone)]
struct AppState {
    pool: Arc<Pool<MySql>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the number of threads available and create that many workers.
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
        App::new().app_data(web::Data::new(app_state.clone()))
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 80))?
    .bind("127.0.0.1")?
    .workers(workers_available)
    .run()
    .await
}
