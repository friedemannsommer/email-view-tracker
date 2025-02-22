use std::str::FromStr;

use sea_orm::{ActiveModelTrait, ActiveValue};

use crate::{
    database::{
        DatabaseError,
        access::{AccessError, add_user, get_user, get_user_by_name},
        connection::{ConnectError, get_database_connection},
    },
    model::config::UserConfig,
    utility::password::hash_password,
};

#[derive(thiserror::Error, Debug)]
pub enum UserOperationError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    Hashing(#[from] argon2::password_hash::Error),
    #[error("{0}")]
    Identity(String),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}

pub async fn fetch_user(
    database: &sea_orm::DatabaseConnection,
    identity: &actix_identity::Identity,
) -> Result<entity::user::ActiveModel, UserOperationError> {
    get_user(
        database,
        uuid::Uuid::from_str(
            &identity
                .id()
                .map_err(|err| UserOperationError::Identity(err.to_string()))?,
        )?,
    )
    .await
    .map_err(map_access_error)
}

pub async fn create_user(user_config: UserConfig) -> Result<uuid::Uuid, UserOperationError> {
    let database = get_database_connection(&user_config.database_url, user_config.log_level)
        .await
        .map_err(map_connect_error)?;
    let timestamp = time::OffsetDateTime::now_utc();
    let user = entity::user::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(user_config.username),
        password: ActiveValue::Set(
            hash_password(&user_config.password).map_err(UserOperationError::from)?,
        ),
        created_at: ActiveValue::Set(timestamp),
        updated_at: ActiveValue::Set(timestamp),
    };

    add_user(&database, user).await.map_err(map_access_error)
}

pub async fn change_user_password(
    user_config: UserConfig,
) -> Result<uuid::Uuid, UserOperationError> {
    let database = get_database_connection(&user_config.database_url, user_config.log_level)
        .await
        .map_err(map_connect_error)?;
    let timestamp = time::OffsetDateTime::now_utc();
    let mut user = get_user_by_name(&database, &user_config.username)
        .await
        .map_err(map_access_error)?;

    user.password =
        ActiveValue::Set(hash_password(&user_config.password).map_err(UserOperationError::from)?);
    user.updated_at = ActiveValue::Set(timestamp);

    Ok(user
        .save(&database)
        .await
        .map_err(|err| {
            UserOperationError::Database(DatabaseError::Access(AccessError::Database(err)))
        })?
        .id
        .unwrap())
}

fn map_connect_error(err: ConnectError) -> UserOperationError {
    UserOperationError::Database(DatabaseError::from(err))
}

fn map_access_error(err: AccessError) -> UserOperationError {
    UserOperationError::Database(DatabaseError::Access(err))
}
