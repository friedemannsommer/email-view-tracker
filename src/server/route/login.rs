use crate::server::template::login;

#[derive(serde::Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[actix_web::get("/")]
pub async fn get_login() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .insert_header((
            actix_web::http::header::CONTENT_TYPE,
            "text/html; charset=utf-8",
        ))
        .body(login::template())
}

#[actix_web::post("/")]
pub async fn post_login(body: actix_web::web::Form<LoginData>) -> actix_web::HttpResponse {
    log::debug!("{:?}", body);

    actix_web::HttpResponse::Ok()
        .insert_header((
            actix_web::http::header::CONTENT_TYPE,
            "text/html; charset=utf-8",
        ))
        .body(login::template())
}
