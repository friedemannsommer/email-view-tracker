use sea_orm::{EntityTrait, QueryFilter};

#[derive(thiserror::Error, Debug)]
pub enum AccessError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
    #[error("Entity with ID ({0}) doesn't exist")]
    NotFound(uuid::Uuid),
    #[error("User with name ({0}) doesn't exist")]
    UserNotFound(String),
}

pub async fn get_user(
    db: &sea_orm::DatabaseConnection,
    user_id: uuid::Uuid,
) -> Result<entity::user::ActiveModel, AccessError> {
    let user_opt = entity::user::Entity::find_by_id(user_id).one(db).await?;

    if let Some(user) = user_opt {
        return Ok(user.into());
    }

    Err(AccessError::NotFound(user_id))
}

pub async fn get_user_by_name(
    db: &sea_orm::DatabaseConnection,
    username: &str,
) -> Result<entity::user::ActiveModel, AccessError> {
    let user_opt = entity::user::Entity::find()
        .filter(sea_orm::sea_query::expr::Expr::col(entity::user::Column::Name).eq(username))
        .one(db)
        .await?;

    if let Some(user) = user_opt {
        return Ok(user.into());
    }

    Err(AccessError::UserNotFound(username.to_string()))
}

pub async fn add_user(
    db: &sea_orm::DatabaseConnection,
    mut user: entity::user::ActiveModel,
) -> Result<uuid::Uuid, AccessError> {
    user.id = sea_orm::entity::ActiveValue::Set(uuid::Uuid::new_v4());

    Ok(entity::user::Entity::insert(user)
        .exec(db)
        .await?
        .last_insert_id)
}

pub async fn get_tracker(
    db: &sea_orm::DatabaseConnection,
    user_id: uuid::Uuid,
    tracker_id: uuid::Uuid,
) -> Result<entity::tracker::ActiveModel, AccessError> {
    let tracker_opt = entity::tracker::Entity::find_by_id(tracker_id)
        .filter(sea_orm::sea_query::expr::Expr::col(entity::tracker::Column::UserId).eq(user_id))
        .one(db)
        .await?;

    if let Some(tracker) = tracker_opt {
        return Ok(tracker.into());
    }

    Err(AccessError::NotFound(tracker_id))
}

pub async fn get_tracker_unauthorized(
    db: &sea_orm::DatabaseConnection,
    tracker_id: uuid::Uuid,
) -> Result<entity::tracker::ActiveModel, AccessError> {
    let tracker_opt = entity::tracker::Entity::find_by_id(tracker_id)
        .one(db)
        .await?;

    if let Some(tracker) = tracker_opt {
        return Ok(tracker.into());
    }

    Err(AccessError::NotFound(tracker_id))
}

pub async fn add_tracker(
    db: &sea_orm::DatabaseConnection,
    mut tracker: entity::tracker::ActiveModel,
) -> Result<uuid::Uuid, AccessError> {
    tracker.id = sea_orm::entity::ActiveValue::Set(uuid::Uuid::new_v4());

    Ok(entity::tracker::Entity::insert(tracker)
        .exec(db)
        .await?
        .last_insert_id)
}
