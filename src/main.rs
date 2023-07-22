use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template; // bring trait in scope
use cmanager::{self, Comment};


#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] 
pub struct CommentTemplate {
    comments: Vec<cmanager::Comment>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async { CommentTemplate {comments: cmanager::get_all()}}))
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