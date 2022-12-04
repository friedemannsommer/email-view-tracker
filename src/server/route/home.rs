use std::str::FromStr;

use sea_orm::{CursorTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{server::template::home, utility::user::fetch_user};

use super::shared::{html_response, server_error};

#[derive(serde::Deserialize, Debug)]
pub struct PaginationQuery {
    after: Option<String>,
    before: Option<String>,
}

#[derive(thiserror::Error, Debug)]
enum PaginationError {
    #[error(transparent)]
    DateTime(#[from] chrono::format::ParseError),
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

#[actix_web::get("/home")]
pub async fn get_home(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user: actix_identity::Identity,
    pagination: actix_web::web::Query<PaginationQuery>,
) -> actix_web::HttpResponse {
    let user = match fetch_user(&database, &user).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };
    let trackers = match fetch_paginated_trackers(&database, *user.id.as_ref(), &pagination).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            Vec::with_capacity(0)
        }
    };

    html_response(home::template(&user, &trackers))
}

async fn fetch_paginated_trackers(
    database: &sea_orm::DatabaseConnection,
    user_id: uuid::Uuid,
    pagination: &PaginationQuery,
) -> Result<Vec<entity::tracker::Model>, PaginationError> {
    let mut cursor = entity::tracker::Entity::find()
        .filter(sea_orm::sea_query::Expr::col(entity::tracker::Column::UserId).eq(user_id))
        .order_by_desc(entity::tracker::Column::CreatedAt)
        .cursor_by(entity::tracker::Column::CreatedAt);

    if let Some(timestamp) = pagination.before.as_ref() {
        cursor.before(chrono::DateTime::<chrono::Utc>::from_str(timestamp)?.naive_utc());
    } else if let Some(timestamp) = pagination.after.as_ref() {
        cursor.after(chrono::DateTime::<chrono::Utc>::from_str(timestamp)?.naive_utc());
    }

    Ok(cursor.first(10).all(database).await?)
}
