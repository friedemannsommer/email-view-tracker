#[derive(Debug)]
pub enum SocketListener {
    Tcp(std::net::SocketAddr),
    #[cfg(unix)]
    Unix(std::path::PathBuf),
}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub listen: SocketListener,
    pub log_level: log::LevelFilter,
    pub worker_count: u8,
}
