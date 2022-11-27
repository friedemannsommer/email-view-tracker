use crate::model::cli::SocketListener;

pub trait LogConfig {
    fn get_log_level(&self) -> log::LevelFilter;
}

#[derive(Debug)]
pub struct ServerConfig {
    pub bind_address: SocketListener,
    pub database_url: String,
    pub log_level: log::LevelFilter,
    pub worker_count: u8,
}

#[derive(Debug)]
pub struct CliConfig {
    pub database_url: String,
    pub log_level: log::LevelFilter,
}

impl LogConfig for &ServerConfig {
    fn get_log_level(&self) -> log::LevelFilter {
        self.log_level
    }
}

impl LogConfig for &CliConfig {
    fn get_log_level(&self) -> log::LevelFilter {
        self.log_level
    }
}
