use sea_orm::{ActiveModelTrait, ActiveValue};

use crate::{
    database::access::get_tracker_unauthorized,
    server::template::tracker,
    utility::{
        tracker::{create_tracker, fetch_tracker},
        user::fetch_user,
    },
};

use super::shared::{html_response, server_error};

#[derive(serde::Deserialize, Debug)]
pub struct TrackerData {
    pub name: String,
}

struct TrackerRequest {
    pub user: entity::user::ActiveModel,
    pub tracker: Option<entity::tracker::ActiveModel>,
}

#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error(transparent)]
    User(#[from] crate::utility::user::UserOperationError),
    #[error(transparent)]
    Tracker(#[from] crate::utility::tracker::TrackerOperationError),
}

const BEACON_GIF: [u8; 35] = [
    71, 73, 70, 56, 55, 97, 1, 0, 1, 0, 128, 0, 0, 252, 106, 108, 0, 0, 0, 44, 0, 0, 0, 0, 1, 0, 1,
    0, 0, 2, 2, 68, 1, 0, 59,
];

#[actix_web::get("/tracker/create")]
pub async fn get_create_tracker(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user_id: actix_identity::Identity,
    req_info: actix_web::dev::ConnectionInfo,
) -> actix_web::HttpResponse {
    let data = match fetch_request_data(&database, &user_id, None).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };

    tracker_response(
        &data.user,
        None,
        req_info.scheme() == "https",
        req_info.host(),
    )
}

#[actix_web::post("/tracker/create")]
pub async fn post_create_tracker(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user_id: actix_identity::Identity,
    body: actix_web::web::Form<TrackerData>,
) -> actix_web::HttpResponse {
    let tracker_id = match create_tracker(&database, &user_id, body.name.clone()).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };

    actix_web::HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!("/tracker/update/{tracker_id}"),
        ))
        .finish()
}

#[actix_web::get("/tracker/update/{id}")]
pub async fn get_update_tracker(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user_id: actix_identity::Identity,
    tracker_id: actix_web::web::Path<uuid::Uuid>,
    req_info: actix_web::dev::ConnectionInfo,
) -> actix_web::HttpResponse {
    let data = match fetch_request_data(&database, &user_id, Some(tracker_id.to_owned())).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };

    tracker_response(
        &data.user,
        data.tracker.as_ref(),
        req_info.scheme() == "https",
        req_info.host(),
    )
}

#[actix_web::post("/tracker/update/{id}")]
pub async fn post_update_tracker(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user_id: actix_identity::Identity,
    tracker_id: actix_web::web::Path<uuid::Uuid>,
    body: actix_web::web::Form<TrackerData>,
    req_info: actix_web::dev::ConnectionInfo,
) -> actix_web::HttpResponse {
    let data = match fetch_request_data(&database, &user_id, Some(tracker_id.to_owned())).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };
    let tracker_name = body.name.trim();
    let mut tracker = data.tracker.unwrap();

    if tracker.name.as_ref() != tracker_name {
        tracker.name = ActiveValue::Set(tracker_name.to_string());
    }

    if tracker.is_changed() {
        tracker.updated_at = ActiveValue::Set(time::OffsetDateTime::now_utc());

        match tracker.save(database.as_ref()).await {
            Ok(result) => tracker = result,
            Err(error) => {
                log::error!("{:?}", error);
                return server_error();
            }
        }
    }

    tracker_response(
        &data.user,
        Some(&tracker),
        req_info.scheme() == "https",
        req_info.host(),
    )
}

#[actix_web::get("/tracker/delete/{id}")]
pub async fn get_delete_tracker(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    user_id: actix_identity::Identity,
    tracker_id: actix_web::web::Path<uuid::Uuid>,
) -> actix_web::HttpResponse {
    let tracker = match fetch_tracker(&database, &user_id, tracker_id.to_owned()).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };

    match tracker.delete(database.as_ref()).await {
        Ok(_) => actix_web::HttpResponse::SeeOther()
            .insert_header((actix_web::http::header::LOCATION, "/home"))
            .finish(),
        Err(error) => {
            log::error!("{:?}", error);
            server_error()
        }
    }
}

#[actix_web::get("/track/{id}")]
pub async fn get_track_impression(
    database: actix_web::web::Data<sea_orm::DatabaseConnection>,
    tracker_id: actix_web::web::Path<uuid::Uuid>,
) -> actix_web::HttpResponse {
    let mut tracker = match get_tracker_unauthorized(&database, tracker_id.to_owned()).await {
        Ok(val) => val,
        Err(error) => {
            log::error!("{:?}", error);
            return server_error();
        }
    };

    tracker.views = if tracker.views.is_not_set() {
        ActiveValue::Set(1)
    } else {
        ActiveValue::Set(tracker.views.unwrap() + 1)
    };

    if let Err(error) = tracker.update(database.as_ref()).await {
        log::error!("{:?}", error);
        return server_error();
    }

    actix_web::HttpResponse::Ok()
        .content_type("image/gif")
        .body(BEACON_GIF.as_ref())
}

async fn fetch_request_data(
    database: &sea_orm::DatabaseConnection,
    user_id: &actix_identity::Identity,
    tracker_id_opt: Option<uuid::Uuid>,
) -> Result<TrackerRequest, OperationError> {
    Ok(TrackerRequest {
        user: fetch_user(database, user_id).await?,
        tracker: if let Some(tracker_id) = tracker_id_opt {
            Some(fetch_tracker(database, user_id, tracker_id).await?)
        } else {
            None
        },
    })
}

fn tracker_response(
    user: &entity::user::ActiveModel,
    tracker: Option<&entity::tracker::ActiveModel>,
    is_ssl: bool,
    hostname: &str,
) -> actix_web::HttpResponse {
    html_response(tracker::template(user, tracker, is_ssl, hostname))
}
