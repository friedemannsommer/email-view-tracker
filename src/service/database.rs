use std::borrow::Cow;

use mysql::params;
use mysql::prelude::Queryable;

use crate::model::database::{Tracker, User};

#[derive(Debug, Clone)]
pub struct Database {
    pool: mysql::Pool,
}

#[derive(thiserror::Error, Debug)]
pub enum DbError<'database_url> {
    #[error("Unsupported database type: {0}")]
    Unsupported(&'database_url str),
    #[error(transparent)]
    Driver(#[from] mysql::error::Error),
}

pub fn get_database(database_url: &str) -> Result<Database, DbError<'_>> {
    if database_url.starts_with("mysql://") {
        Ok(Database {
            pool: mysql::Pool::new(database_url)?,
        })
    } else {
        Err(DbError::Unsupported(database_url))
    }
}

impl Database {
    pub fn create_tracker<'name>(
        &self,
        name: Cow<'name, str>,
        user: &User<'_, '_>,
    ) -> Result<Tracker<'name>, DbError<'_>> {
        let timestamp = chrono::Utc::now();
        let tracker = Tracker {
            created_at: timestamp.naive_utc(),
            id: uuid::Uuid::new_v4(),
            name,
            updated_at: timestamp.naive_utc(),
            user_id: user.id,
            views: Default::default(),
        };
        let mut connection = self.pool.try_get_conn(10_000)?;

        connection.exec_drop(
            r#"INSERT INTO trackers (created_at, id, name, updated_at, user_id, views)
        VALUES (:created_at, :id, :name, :updated_at, :user_id, :views)"#,
            mysql::params! {
                "created_at" => tracker.created_at.timestamp(),
                "id" => tracker.id.as_bytes(),
                "name" => tracker.name.as_ref(),
                "updated_at" => &tracker.updated_at.timestamp(),
                "user_id" => tracker.user_id.as_bytes(),
                "views" => tracker.views
            },
        )?;

        Ok(tracker)
    }

    pub fn create_user<'name, 'password>(
        &self,
        name: Cow<'name, str>,
        password: Cow<'password, str>,
    ) -> Result<User<'name, 'password>, DbError<'_>> {
        let timestamp = chrono::Utc::now();
        let user = User {
            created_at: timestamp.naive_utc(),
            id: uuid::Uuid::new_v4(),
            name,
            password,
            updated_at: timestamp.naive_utc(),
        };
        let mut connection = self.pool.try_get_conn(10_000)?;

        connection.exec_drop(
            r#"INSERT INTO users (created_at, id, name, password, updated_at)
        VALUES (:created_at, :id, :name, :password, :updated_at)"#,
            mysql::params! {
                "created_at" => user.created_at.timestamp(),
                "id" => user.id.as_bytes(),
                "name" => user.name.as_ref(),
                "password" => user.password.as_ref(),
                "updated_at" => user.updated_at.timestamp(),
            },
        )?;

        Ok(user)
    }

    pub fn increment_tracker_view_count(&self, tracker_id: &uuid::Uuid) -> Result<(), DbError<'_>> {
        let mut connection = self.pool.try_get_conn(10_000)?;

        connection.exec_drop(
            r#"UPDATE trackers
        SET trackers.views = trackers.views + 1
        WHERE trackers.id = :tracker_id"#,
            mysql::params! {
                "tracker_id" => tracker_id.as_bytes(),
            },
        )?;

        Ok(())
    }

    pub fn update_tracker<'name>(
        &self,
        tracker_id: &uuid::Uuid,
        name: Cow<'name, str>,
    ) -> Result<(), DbError<'_>> {
        let mut connection = self.pool.try_get_conn(10_000)?;

        connection.exec_drop(
            r#"UPDATE trackers
        SET trackers.name = :name
        WHERE trackers.id = :tracker_id"#,
            mysql::params! {
                "name" => name.as_ref(),
                "tracker_id" => tracker_id.as_bytes(),
            },
        )?;

        Ok(())
    }

    pub fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, DbError> {
        let mut connection = self.pool.try_get_conn(10_000)?;
        let user_opt: Option<User<'_, '_>> = connection.exec_first(
            r#"SELECT (created_at, name, updated_at)
        FROM users
        WHERE users.id = :user_id"#,
            mysql::params! {
                "user_id" => user_id.as_bytes()
            },
        )?;
    }
}
