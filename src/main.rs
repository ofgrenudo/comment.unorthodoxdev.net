use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use actix_web::{web::{self, trace}, App, HttpServer, Responder, HttpRequest, http::header::HeaderValue};
use askama::Template; // bring trait in scope
use cmanager;
use serde::Deserialize;
use log4rs;
use log::*;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] 
pub struct CommentTemplate {
    comments: Vec<cmanager::Comment>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct FormData {
    username: String,
    comment: String,
}

async fn new(web::Form(form): web::Form<FormData>, req: HttpRequest) -> impl Responder {
    // let ip = req.peer_addr().unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(6, 6, 6, 6)), 666)).ip();
    let default_ip = HeaderValue::from_static("127.0.0.1"); // fail point one
    let mut ip_ref = req.headers().get("X-Forwarded-For").unwrap_or(&default_ip);
    let ip = ip_ref.to_str().unwrap_or("6.6.6.6"); // fail point two, possibly unneeded
    let username = form.username;
    let comment = form.comment;

    debug!("Recieved create new comment request from: {} with username: {}, and comment: {}", ip, username, comment);
    let new_comment_result = cmanager::new(String::from(ip), username, comment);
    debug!("Submitted new comment {:?}", new_comment_result);

    debug!("Redirecting to the root /comment/");
    web::redirect("/comment/new", "/comment/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    debug!("running now on https://0.0.0.0:8080/comment/");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/comment/").to(|| async { 
                CommentTemplate {comments: cmanager::get_all()
            }}))
            .route("/comment/new", web::post().to(new))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
