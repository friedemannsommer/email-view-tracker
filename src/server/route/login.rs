use actix_web::HttpMessage;

use crate::{
    database::access::{get_user_by_name, AccessError},
    server::{lib::ip_session::IpSession, template::login},
    utility::password::verify_password,
};

#[derive(serde::Deserialize, Debug)]
pub struct LoginData {
    pub csrf_token: String,
    pub password: String,
    pub username: String,
}

const CSRF_COOKIE_NAME: &str = "csrf";

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
    ip_session: actix_web::web::Data<IpSession>,
    request: actix_web::HttpRequest,
    body: actix_web::web::Form<LoginData>,
) -> actix_web::HttpResponse {
    if !validate_csrf(&request, &body) {
        return actix_web::HttpResponse::Unauthorized().finish();
    }

    let remote_ip_opt = get_remote_ip(&request);

    if let Some(remote_ip) = remote_ip_opt.as_ref() {
        if ip_session.ip_blocked(remote_ip) {
            return actix_web::HttpResponse::Unauthorized().finish();
        }
    }

    let user_res = get_user_by_name(&database, &body.username).await;

    if let Err(error) = user_res {
        log::error!("{:?}", error);

        if let Some(remote_ip) = remote_ip_opt {
            if let AccessError::UserNotFound(_) = error {
                ip_session.increment_ip(remote_ip)
            }
        }

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

        if let Some(remote_ip) = remote_ip_opt.as_ref() {
            ip_session.reset_ip(remote_ip)
        }

        return redirect_user();
    } else if let Some(remote_ip) = remote_ip_opt {
        ip_session.increment_ip(remote_ip)
    }

    login_response()
}

fn validate_csrf(request: &actix_web::HttpRequest, form_data: &LoginData) -> bool {
    if let Some(cookie) = request.cookie(CSRF_COOKIE_NAME) {
        return cookie.value() == form_data.csrf_token;
    }

    false
}

fn get_remote_ip(request: &actix_web::HttpRequest) -> Option<String> {
    let connection_info = request.connection_info();

    if let Some(real_ip) = connection_info.realip_remote_addr() {
        return Some(String::from(real_ip));
    }

    connection_info.peer_addr().map(String::from)
}

fn login_response() -> actix_web::HttpResponse {
    let csrf_res = crate::server::lib::csrf::generate_token();

    if let Ok(csrf_token) = csrf_res {
        actix_web::HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build(CSRF_COOKIE_NAME, &csrf_token)
                    .secure(true)
                    .http_only(true)
                    .same_site(actix_web::cookie::SameSite::Strict)
                    .finish(),
            )
            .content_type("text/html; charset=utf-8")
            .body(login::template(&csrf_token))
    } else {
        log::error!("{:?}", csrf_res.unwrap_err());
        actix_web::HttpResponse::InternalServerError().finish()
    }
}

fn redirect_user() -> actix_web::HttpResponse {
    actix_web::HttpResponse::SeeOther()
        .cookie(
            actix_web::cookie::Cookie::build(CSRF_COOKIE_NAME, "")
                .expires(actix_web::cookie::time::OffsetDateTime::UNIX_EPOCH)
                .finish(),
        )
        .insert_header((actix_web::http::header::LOCATION, "/home"))
        .finish()
}
