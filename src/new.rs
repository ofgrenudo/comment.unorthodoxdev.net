use actix_web::{HttpRequest, web, http::header::HeaderValue, Responder, route};
use log::debug;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct FormData {
    username: String,
    comment: String,
}

#[route("/comment/new/", method="POST", method="PUT")]
pub async fn comment(web::Form(form): web::Form<FormData>, req: HttpRequest) -> impl Responder {
    // let ip = req.peer_addr().unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(6, 6, 6, 6)), 666)).ip();
    let default_ip = HeaderValue::from_static("127.0.0.1"); // fail point one
    let ip_ref = req.headers().get("X-Forwarded-For").unwrap_or(&default_ip);
    let ip = ip_ref.to_str().unwrap_or("6.6.6.6"); // fail point two, possibly unneeded
    let username = form.username;
    let comment = form.comment;

    debug!("Recieved create new comment request from: {} with username: {}, and comment: {}", ip, username, comment);
    let new_comment_result = cmanager::new(String::from(ip), username, comment);
    debug!("Submitted new comment {:?}", new_comment_result);

    debug!("Redirecting to the root /comment/get/latest");
    web::redirect("/comment/new", "/comment/get/latest")
}
