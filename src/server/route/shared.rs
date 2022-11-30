pub fn html_response(html: String) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .insert_header((
            actix_web::http::header::CONTENT_TYPE,
            "text/html; charset=utf-8",
        ))
        .body(html)
}

pub fn server_error() -> actix_web::HttpResponse {
    actix_web::HttpResponse::InternalServerError().finish()
}
