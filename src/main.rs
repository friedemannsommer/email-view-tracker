#![deny(
    clippy::correctness,
    clippy::style,
    keyword_idents,
    macro_use_extern_crate,
    non_ascii_idents,
    nonstandard_style,
    noop_method_call,
    pointer_structural_match,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_crate_dependencies
)]
#![warn(
    clippy::cargo,
    clippy::complexity,
    clippy::perf,
    clippy::suspicious,
    rust_2018_idioms,
    unused
)]
#![allow(clippy::multiple_crate_versions)]

use actix_identity as _;
use actix_session as _;
use actix_web as _;
use chrono as _;
use diesel as _;
use markup as _;
use uuid as _;

mod models;

fn main() {
    let config = get_config();

    init_logging(&config);
    log::debug!("{:?}", &config);
    // TODO: implement DB pool and HTTP server
}

fn get_config() -> models::config::Config {
    use clap::Parser;

    let args: models::cli::Cli = models::cli::Cli::parse();

    models::config::Config {
        database_url: args.database_url,
        listen: parse_socket_listener(&args.listen),
        log_level: args.log_level,
        worker_count: args.worker_count,
    }
}

fn parse_socket_listener(input: &str) -> models::config::SocketListener {
    use std::str::FromStr;

    if let Ok(address) = std::net::SocketAddr::from_str(input) {
        return models::config::SocketListener::Tcp(address);
    }

    #[cfg(unix)]
    if let Ok(path) = std::path::PathBuf::from_str(input) {
        return models::config::SocketListener::Unix(path);
    }

    panic!("Listener could not be parsed: '{}'", input)
}

fn init_logging(config: &models::config::Config) {
    if config.log_level != log::LevelFilter::Off {
        let mut logger = fern::Dispatch::new().level(config.log_level);

        if config.log_level != log::LevelFilter::Error {
            logger = logger.chain(
                fern::Dispatch::new()
                    .filter(|meta| meta.level() != log::LevelFilter::Error)
                    .chain(std::io::stdout()),
            )
        }

        logger
            .chain(
                fern::Dispatch::new()
                    .level(log::LevelFilter::Error)
                    .chain(std::io::stderr()),
            )
            .apply()
            .expect("logging subscriber registration failed");
    } else {
        log::set_max_level(config.log_level);
    }
}
