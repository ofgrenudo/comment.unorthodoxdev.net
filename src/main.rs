use std::path::Path;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use log::*;
use log4rs;

mod get;
mod new;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    info!("running now on https://0.0.0.0:8080/");

    if cfg!(debug_assertions) {
        warn!("CORS is disabled in DEBUG mode.");
        HttpServer::new(|| {
            App::new()
                .service(get::default)
                .service(get::latest)
                .service(get::all)
                .service(get::by_id)
                .service(get::by_username)
                .service(get::by_post)
                .service(get::by_post_raw)
                .service(new::comment)
                .service(new::comment_on_post)
                .service(new::post_any_data)
                .service(actix_files::Files::new(
                    "/comment/static/",
                    Path::new(&format!("./static")),
                ))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    } else {
        HttpServer::new(|| {
            warn!("CORS is enabled in RELEASE mode.");
            let cors = Cors::default()
                .allowed_origin("http://localhost:1313")
                .allowed_origin("https://unorthodoxdev.net")
                .allowed_origin("http://unorthodoxdev.net")
                .allowed_methods(vec!["GET", "POST", "PUT"])
                .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                .max_age(3600);

            App::new()
                .wrap(cors)
                .service(get::default)
                .service(get::latest)
                .service(get::all)
                .service(get::by_id)
                .service(get::by_username)
                .service(get::by_post)
                .service(get::by_post_raw)
                .service(new::comment)
                .service(new::comment_on_post)
                .service(new::post_any_data)
                .service(actix_files::Files::new(
                    "/comment/static/",
                    Path::new(&format!("./static")),
                ))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    }
}
