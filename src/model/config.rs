use crate::model::cli::SocketListener;

pub trait LogConfig {
    fn get_log_level(&self) -> log::LevelFilter;
}

#[derive(Debug)]
pub struct ServerConfig {
    pub bind_address: SocketListener,
    pub cookie_secret: String,
    pub database_url: String,
    pub log_level: log::LevelFilter,
    pub password_secret: String,
    pub worker_count: u8,
}

#[derive(Debug)]
pub struct MigrateConfig {
    pub database_url: String,
    pub log_level: log::LevelFilter,
}

#[derive(Debug)]
pub struct UserConfig {
    pub database_url: String,
    pub log_level: log::LevelFilter,
    pub password: String,
    pub password_secret: String,
    pub username: String,
}

impl LogConfig for &ServerConfig {
    fn get_log_level(&self) -> log::LevelFilter {
        self.log_level
    }
}

impl LogConfig for &MigrateConfig {
    fn get_log_level(&self) -> log::LevelFilter {
        self.log_level
    }
}

impl LogConfig for &UserConfig {
    fn get_log_level(&self) -> log::LevelFilter {
        self.log_level
    }
}
