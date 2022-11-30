use actix_web::HttpMessage;

use crate::{
    database::access::get_user_by_name, server::template::login, utility::password::verify_password,
};

use super::shared::html_response;

#[derive(serde::Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[actix_web::get("/")]
pub async fn get_login(user: Option<actix_identity::Identity>) -> actix_web::HttpResponse {
    if user.is_some() {
        redirect_user()
    } else {
        login_response()
    }
}

#[actix_web::post("/")]
pub async fn post_login(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    request: actix_web::HttpRequest,
    body: actix_web::web::Form<LoginData>,
) -> actix_web::HttpResponse {
    let user_res = get_user_by_name(&database, &body.username).await;

    if let Err(error) = user_res {
        log::error!("{:?}", error);

        return login_response();
    }

    let user = user_res.unwrap();

    if verify_password(&body.password, user.password.as_ref()) {
        if let Err(error) =
            actix_identity::Identity::login(&request.extensions(), user.id.as_ref().to_string())
        {
            log::error!("{:?}", error);

            return login_response();
        }

        return redirect_user();
    }

    login_response()
}

fn login_response() -> actix_web::HttpResponse {
    html_response(login::template())
}

fn redirect_user() -> actix_web::HttpResponse {
    actix_web::HttpResponse::SeeOther()
        .insert_header((actix_web::http::header::LOCATION, "/home"))
        .finish()
}
