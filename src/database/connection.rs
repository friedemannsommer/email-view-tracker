use std::{borrow::Cow, time::Duration};

#[derive(thiserror::Error, Debug)]
pub enum ConnectError {
    #[error(transparent)]
    SqlX(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    IntParse(#[from] std::num::ParseIntError),
}

pub async fn get_database_connection(
    database_url: &str,
    log_level: log::LevelFilter,
) -> Result<sea_orm::DatabaseConnection, ConnectError> {
    let url = url::Url::parse(database_url).map_err(ConnectError::from)?;
    let mut connect_options = sea_orm::ConnectOptions::new(database_url.to_string());

    if log_level != log::LevelFilter::Off {
        connect_options.sqlx_logging(true);
        connect_options.sqlx_logging_level(log_level);
    }

    for (name, value) in url.query_pairs() {
        use std::str::FromStr;

        match name {
            Cow::Borrowed("max_connections") => {
                connect_options.max_connections(u32::from_str(value.as_ref())?);
            }
            Cow::Borrowed("min_connections") => {
                connect_options.min_connections(u32::from_str(value.as_ref())?);
            }
            Cow::Borrowed("connect_timeout") => {
                connect_options
                    .connect_timeout(Duration::from_secs(u64::from_str(value.as_ref())?));
            }
            Cow::Borrowed("acquire_timeout") => {
                connect_options
                    .acquire_timeout(Duration::from_secs(u64::from_str(value.as_ref())?));
            }
            Cow::Borrowed("idle_timeout") => {
                connect_options.idle_timeout(Duration::from_secs(u64::from_str(value.as_ref())?));
            }
            Cow::Borrowed("max_lifetime") => {
                connect_options.max_lifetime(Duration::from_secs(u64::from_str(value.as_ref())?));
            }
            _ => {}
        };
    }

    sea_orm::Database::connect(connect_options)
        .await
        .map_err(ConnectError::from)
}
