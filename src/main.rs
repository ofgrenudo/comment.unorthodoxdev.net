use std::path::Path;

use actix_web::{App, HttpServer};
use log4rs;
use log::*;

mod new;
mod get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    info!("running now on https://0.0.0.0:8080/comment/");
    HttpServer::new(|| {
        App::new()
        .service(get::default)
        .service(get::latest)
        .service(get::all)
        .service(get::by_id)
        .service(get::by_username)
        .service(new::comment)
        .service(new::comment_on_post)
        .service(actix_files::Files::new(
            "/comment/static/",
            Path::new(&format!("./static")),
        ))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
