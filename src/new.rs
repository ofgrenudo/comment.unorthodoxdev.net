use actix_web::{http::{header::{ContentType, HeaderValue}, StatusCode}, route, web, HttpRequest, HttpResponse, Responder};
use log::debug;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct FormData {
    username: String,
    comment: String,
    post_url: String,
}

#[route("/new/", method="POST", method="PUT")]
pub async fn comment(web::Form(form): web::Form<FormData>, req: HttpRequest) -> impl Responder {
    // let ip = req.peer_addr().unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(6, 6, 6, 6)), 666)).ip();
    let default_ip = HeaderValue::from_static("127.0.0.1"); // fail point one
    let ip_ref = req.headers().get("X-Forwarded-For").unwrap_or(&default_ip);
    let ip = ip_ref.to_str().unwrap_or("6.6.6.6"); // fail point two, possibly unneeded
    let username = form.username;
    let comment = form.comment;

    debug!("Recieved create new comment request from: {} with username: {}, and comment: {}", ip, username, comment);
    let new_comment_result = cmanager::new::comment(String::from(ip), username, comment);
    debug!("Submitted new comment {:?}", new_comment_result);

    debug!("Redirecting to the root /comment/get/latest");
    web::redirect("/comment/new", "/comment/get/latest")
}

#[route("/new/post", method="POST", method="PUT")]
pub async fn comment_on_post(web::Form(form): web::Form<FormData>, req: HttpRequest) -> impl Responder  {
    debug!("{:#?}", form);
    let default_ip = HeaderValue::from_static("127.0.0.1");
    let ip_ref = req.headers().get("X-Forwarded-For").unwrap_or(&default_ip);
    let ip = ip_ref.to_str().unwrap_or("6.6.6.6");
    
    let username = form.username;
    let comment_text = form.comment;  // Renamed from `comment`
    let post_url = form.post_url;

    debug!(
        "Received create new comment request for a post with the URL: {}, username: {}, and comment: {}",
        post_url, username, comment_text
    );

    let new_comment_result = cmanager::new::comment_on_post(String::from(ip), username, comment_text, post_url.clone());
    debug!("Submitted new comment {:?}", new_comment_result);
    debug!("Redirecting to the root /comment/get/latest");
    // Return an acknowledgment response
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}

#[route("/debug", method="POST")]
pub async fn post_any_data(web::Form(form): web::Form<FormData>, req: HttpRequest) -> impl Responder {
    debug!("{:#?}", form);
    let default_ip = HeaderValue::from_static("127.0.0.1");
    let ip_ref = req.headers().get("X-Forwarded-For").unwrap_or(&default_ip);
    let ip = ip_ref.to_str().unwrap_or("6.6.6.6");
    
    let username = form.username;
    let comment_text = form.comment;  // Renamed from `comment`
    let post_url = form.post_url;

    debug!(
        "Received create new comment request for a post with the URL: {}, username: {}, and comment: {}",
        post_url, username, comment_text
    );

    let new_comment_result = cmanager::new::comment_on_post(String::from(ip), username, comment_text, post_url);
    debug!("Submitted new comment {:?}", new_comment_result);
    debug!("Redirecting to the root /comment/get/latest");

    web::redirect("/new", "/get/latest")
}