use crate::{server::template::home, utility::user::fetch_user};

use super::shared::{html_response, server_error};

#[actix_web::get("/home")]
pub async fn get_home(
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

    html_response(home::template(&user))
}
