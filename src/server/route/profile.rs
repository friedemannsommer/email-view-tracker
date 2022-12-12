use sea_orm::{ActiveModelTrait, ActiveValue};

use crate::{
    server::template::profile,
    utility::{password::hash_password, user::fetch_user},
};

use super::shared::{html_response, server_error};

#[derive(serde::Deserialize, Debug)]
pub struct ProfileData {
    pub username: String,
    pub password: Option<String>,
}

#[actix_web::get("/profile")]
pub async fn get_profile(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user: actix_identity::Identity,
) -> actix_web::HttpResponse {
    let user = match fetch_user(&database, &user).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };

    profile_response(&user)
}

#[actix_web::post("/profile")]
pub async fn post_profile(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user: actix_identity::Identity,
    body: actix_web::web::Form<ProfileData>,
) -> actix_web::HttpResponse {
    let mut user = match fetch_user(&database, &user).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };
    let username = body.username.trim();

    if user.name.as_ref() != username {
        user.name = ActiveValue::Set(username.to_string());
    }

    if let Some(password) = &body.password {
        let password_trimmed = password.trim();

        if !password_trimmed.is_empty() {
            user.password = match hash_password(password_trimmed) {
                Ok(val) => ActiveValue::Set(val),
                Err(error) => {
                    log::error!("{:?}", error);
                    return server_error();
                }
            }
        }
    }

    if user.is_changed() {
        user.updated_at = ActiveValue::Set(time::OffsetDateTime::now_utc());

        match user.save(database.as_ref()).await {
            Ok(next_user) => user = next_user,
            Err(error) => {
                log::error!("{:?}", error);
                return server_error();
            }
        }
    }

    profile_response(&user)
}

fn profile_response(user: &entity::user::ActiveModel) -> actix_web::HttpResponse {
    html_response(profile::template(user))
}
