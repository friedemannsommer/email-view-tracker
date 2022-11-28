pub async fn redirect_not_found() -> actix_web::HttpResponse {
    actix_web::HttpResponse::TemporaryRedirect()
        .insert_header((actix_web::http::header::LOCATION, "/"))
        .finish()
}
