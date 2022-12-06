#[actix_web::get("/logout")]
pub async fn get_logout(user: actix_identity::Identity) -> actix_web::HttpResponse {
    user.logout();

    actix_web::HttpResponse::SeeOther()
        .insert_header((actix_web::http::header::LOCATION, "/"))
        .finish()
}
