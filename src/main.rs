use std::path::Path;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use log4rs;
use log::*;

mod new;
mod get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();



    info!("running now on https://0.0.0.0:8080/");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1313")
            .allowed_origin("http://localhost:80")
            .allowed_origin("http://localhost:443")
            .allowed_origin("https://unorthodoxdev.net/")
            .allowed_origin("http://unorthodoxdev.net/")
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
