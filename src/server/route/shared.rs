pub fn html_response(html: String) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn server_error() -> actix_web::HttpResponse {
    actix_web::HttpResponse::InternalServerError().finish()
}
