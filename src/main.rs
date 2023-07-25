use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use actix_web::{get, post, web::{self, Form}, App, HttpResponse, HttpServer, Responder, HttpRequest};
use askama::Template; // bring trait in scope
use cmanager::{self, Comment};
use serde::Deserialize;


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

async fn new(web::Form(form): web::Form<FormData>, req: HttpRequest) -> CommentTemplate {
    let ip = req.peer_addr().unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(6, 6, 6, 6)), 666));
    let username = form.username;
    let comment = form.comment;

    let _ = cmanager::new(ip.to_string(), username, comment);

    CommentTemplate {comments: cmanager::get_all()}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async { CommentTemplate {comments: cmanager::get_all()}}))
            .route("/new", web::post().to(new))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() {
//     // cmanager::new("192.168.1.213".to_string(), "jwintersbro".to_string(), "ik bro thanks bro so dope you responded bro!".to_string());
//     let comments = CommentTemplate { comments: cmanager::get_all() };
//     // println!("{:?}", comments);

//     println!("{}", comments.render().unwrap());
// }