use actix_web::{route, HttpResponse, web, Responder};
use askama::Template;
use cmanager::Comment;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] 
pub struct CommentTemplate {
    comments: Vec<Comment>,
}

#[route("/comment/", method="GET", method="POST", method="PUT")]
pub async fn default() -> impl Responder {
    web::redirect("/comment/", "/comment/get/latest")
}

#[route("/comment/get/latest", method="GET", method="POST", method="PUT")]
pub async fn latest() -> HttpResponse {
    let content = CommentTemplate {comments: cmanager::get::get_latest()}
        .render()
        .expect("Error rendering index");

        HttpResponse::Ok().body(content)
}

#[route("/get/all", method="GET", method="POST", method="PUT")]
pub async fn all() -> HttpResponse {
    let content = CommentTemplate {comments: cmanager::get::get_all_comments()}
        .render()
        .expect("Error rendering index");

        HttpResponse::Ok().body(content)
}

#[route("/get/id/{provided_id}", method="GET", method="POST", method="PUT")]
pub async fn by_id(path: web::Path<String>) -> HttpResponse {
    let provided_id = path.into_inner();
    let content = CommentTemplate {comments: cmanager::get::get_one_by_id(provided_id)}
        .render()
        .expect("Error rendering index");

        HttpResponse::Ok().body(content)
}

#[route("/get/username/{username}", method="GET", method="POST", method="PUT")]
pub async fn by_username(path: web::Path<String>) -> HttpResponse {
    let provided_username = path.into_inner();
    let content = CommentTemplate {comments: cmanager::get::get_all_by_username(provided_username)}
        .render()
        .expect("Error rendering index");

        HttpResponse::Ok().body(content)
}

#[route("/get/post/{url}", method="GET", method="POST", method="PUT")]
pub async fn by_post(path: web::Path<String>) -> HttpResponse {
    let provided_url = path.into_inner();
    let content = CommentTemplate {comments: cmanager::get::get_all_on_post(provided_url)}
        .render()
        .expect("Error rendering index");

        HttpResponse::Ok().body(content)
}

#[route("/get/post/raw/{url}", method="GET", method="POST", method="PUT")]
pub async fn by_post_raw(path: web::Path<String>) -> HttpResponse {
    let provided_url = path.into_inner();
    let content = cmanager::get::get_all_on_post(provided_url);
    let json_content = serde_json::to_string(&content).expect("Error serializing content");

    // Return the content as JSON
    HttpResponse::Ok().body(json_content)
}