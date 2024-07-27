#![deny(
    clippy::correctness,
    clippy::style,
    keyword_idents,
    macro_use_extern_crate,
    non_ascii_idents,
    nonstandard_style,
    noop_method_call,
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

use crate::{
    model::config::LogConfig,
    server::serve::start_http_service,
    utility::{
        password::SALT,
        user::{change_user_password, create_user},
    },
};

mod database;
mod model;
mod server;
mod utility;

#[actix_web::main]
async fn main() {
    match model::cli::process_cli() {
        Some(model::cli::CliCommand::HttpServer(config)) => {
            init_logging(&config);
            log::debug!("{:?}", config);
            SALT.set(argon2::password_hash::SaltString::from_b64(&config.password_secret).unwrap())
                .unwrap();
            start_http_service(config).await.unwrap()
        }
        Some(model::cli::CliCommand::MigrateCheck(config)) => {
            init_logging(&config);
            log::debug!("{:?}", config);
            database::migrate::process_database_migrate(database::migrate::MigrationAction::Check(
                config,
            ))
            .await
            .unwrap()
        }
        Some(model::cli::CliCommand::MigrateRun(config)) => {
            init_logging(&config);
            log::debug!("{:?}", config);
            database::migrate::process_database_migrate(database::migrate::MigrationAction::Run(
                config,
            ))
            .await
            .unwrap()
        }
        Some(model::cli::CliCommand::UserCreate(config)) => {
            init_logging(&config);
            log::debug!("{:?}", config);
            SALT.set(argon2::password_hash::SaltString::from_b64(&config.password_secret).unwrap())
                .unwrap();
            log::info!("{:?}", create_user(config).await.unwrap());
        }
        Some(model::cli::CliCommand::UserChangePassword(config)) => {
            init_logging(&config);
            log::debug!("{:?}", config);
            SALT.set(argon2::password_hash::SaltString::from_b64(&config.password_secret).unwrap())
                .unwrap();
            log::info!("{:?}", change_user_password(config).await.unwrap())
        }
        None => {
            unreachable!("No command given")
        }
    }
}

fn init_logging(config: impl LogConfig) {
    let log_level = config.get_log_level();

    if log_level != log::LevelFilter::Off {
        let mut logger = fern::Dispatch::new().level(log_level);

        if log_level != log::LevelFilter::Error {
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
        log::set_max_level(log_level);
    }
}
