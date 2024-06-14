use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    server::{
        model::tracker_paginator::{
            OrderType, TrackerOrderColumn, TrackerPagination, TrackersQuery,
        },
        template::home,
    },
    utility::user::fetch_user,
};

use super::shared::{html_response, server_error};

#[derive(thiserror::Error, Debug)]
enum PaginationError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

#[actix_web::get("/home")]
pub async fn get_home(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user: actix_identity::Identity,
    pagination: actix_web::web::Query<TrackersQuery>,
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
            return server_error();
        }
    };

    html_response(home::template(&user, &trackers))
}

async fn fetch_paginated_trackers<'user_query>(
    database: &sea_orm::DatabaseConnection,
    user_id: uuid::Uuid,
    user_query: &'user_query TrackersQuery,
) -> Result<TrackerPagination<'user_query>, PaginationError> {
    let cursor = entity::tracker::Entity::find()
        .filter(sea_orm::sea_query::Expr::col(entity::tracker::Column::UserId).eq(user_id))
        .order_by(
            match &user_query.order_by {
                Some(column) => match column {
                    TrackerOrderColumn::CreatedAt => entity::tracker::Column::CreatedAt,
                    TrackerOrderColumn::Name => entity::tracker::Column::Name,
                    TrackerOrderColumn::UpdatedAt => entity::tracker::Column::UpdatedAt,
                    TrackerOrderColumn::Views => entity::tracker::Column::Views,
                },
                _ => entity::tracker::Column::CreatedAt,
            },
            match &user_query.order {
                Some(order_by) => match order_by {
                    OrderType::Asc => sea_orm::sea_query::types::Order::Asc,
                    OrderType::Desc => sea_orm::sea_query::types::Order::Desc,
                },
                _ => sea_orm::sea_query::types::Order::Desc,
            },
        )
        .paginate(database, 10);
    let page = user_query.page.unwrap_or_default();
    let pagination = cursor.num_items_and_pages().await?;

    Ok(TrackerPagination {
        entries: if pagination.number_of_items > 0 {
            cursor.fetch_page(page).await?
        } else {
            Vec::with_capacity(0)
        },
        number_of_pages: pagination.number_of_pages,
        page,
        user_query,
    })
}
