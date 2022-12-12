use std::str::FromStr;

use sea_orm::ActiveValue;

use crate::database::{
    access::{add_tracker, get_tracker, AccessError},
    DatabaseError,
};

#[derive(thiserror::Error, Debug)]
pub enum TrackerOperationError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error("{0}")]
    Identity(String),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}

pub async fn fetch_tracker(
    database: &sea_orm::DatabaseConnection,
    identity: &actix_identity::Identity,
    tracker_id: uuid::Uuid,
) -> Result<entity::tracker::ActiveModel, TrackerOperationError> {
    get_tracker(database, user_id_from_identity(identity)?, tracker_id)
        .await
        .map_err(map_access_error)
}

pub async fn create_tracker(
    database: &sea_orm::DatabaseConnection,
    identity: &actix_identity::Identity,
    tracker_name: String,
) -> Result<uuid::Uuid, TrackerOperationError> {
    let timestamp = time::OffsetDateTime::now_utc();
    let tracker = entity::tracker::ActiveModel {
        created_at: ActiveValue::Set(timestamp),
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(tracker_name),
        updated_at: ActiveValue::Set(timestamp),
        user_id: ActiveValue::Set(user_id_from_identity(identity)?),
        views: ActiveValue::Set(0),
    };

    add_tracker(database, tracker)
        .await
        .map_err(map_access_error)
}

fn user_id_from_identity(
    identity: &actix_identity::Identity,
) -> Result<uuid::Uuid, TrackerOperationError> {
    Ok(uuid::Uuid::from_str(&identity.id().map_err(|err| {
        TrackerOperationError::Identity(err.to_string())
    })?)?)
}

fn map_access_error(err: AccessError) -> TrackerOperationError {
    TrackerOperationError::Database(DatabaseError::Access(err))
}
