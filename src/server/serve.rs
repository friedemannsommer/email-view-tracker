use actix_session::config::CookieContentSecurity;
use actix_web::cookie::SameSite;

use crate::{
    database::connection::{get_database_connection, ConnectError},
    model::cli::SocketListener,
    server::lib::header::get_default_headers_middleware,
};

#[derive(thiserror::Error, Debug)]
pub enum HttpServeError {
    #[error(transparent)]
    Database(#[from] ConnectError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub async fn start_http_service(
    config: crate::model::config::ServerConfig,
) -> Result<(), HttpServeError> {
    let database_connection = get_database_connection(&config.database_url, config.log_level)
        .await
        .map_err(HttpServeError::from)?;
    let instance_key = actix_web::cookie::Key::from(config.cookie_secret.as_bytes());
    let ip_session = super::lib::ip_session::IpSession::default();
    let mut http_server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(database_connection.clone()))
            .app_data(actix_web::web::Data::new(ip_session.clone()))
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .wrap(actix_identity::IdentityMiddleware::default())
            .wrap(
                actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    instance_key.clone(),
                )
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Strict)
                .cookie_content_security(CookieContentSecurity::Private)
                .session_lifecycle(
                    actix_session::config::BrowserSession::default()
                        .state_ttl(actix_web::cookie::time::Duration::days(1)),
                )
                .build(),
            )
            .wrap(actix_web::middleware::Condition::new(
                log::log_enabled!(log::Level::Info),
                actix_web::middleware::Logger::new("%a '%r' %s %T"),
            ))
            .wrap(get_default_headers_middleware())
            .service(crate::static_asset_route!(
                "/css/login.css",
                concat!(env!("OUT_DIR"), "/login.css"),
                "text/css"
            ))
            .service(crate::static_asset_route!(
                "/css/user.css",
                concat!(env!("OUT_DIR"), "/user.css"),
                "text/css"
            ))
            .service(crate::static_asset_route!(
                "/fonts/montserrat/regular.woff2",
                "../assets/fonts/montserrat/regular.woff2",
                "font/woff2"
            ))
            .service(crate::static_asset_route!(
                "/fonts/montserrat/regular.woff",
                "../assets/fonts/montserrat/regular.woff",
                "font/woff"
            ))
            .service(crate::static_asset_route!(
                "/fonts/montserrat/bold.woff2",
                "../assets/fonts/montserrat/bold.woff2",
                "font/woff2"
            ))
            .service(crate::static_asset_route!(
                "/fonts/montserrat/bold.woff",
                "../assets/fonts/montserrat/bold.woff",
                "font/woff"
            ))
            .service(super::route::login::get_login)
            .service(super::route::login::post_login)
            .service(super::route::logout::get_logout)
            .service(super::route::home::get_home)
            .service(super::route::profile::get_profile)
            .service(super::route::profile::post_profile)
            .service(super::route::tracker::get_create_tracker)
            .service(super::route::tracker::post_create_tracker)
            .service(super::route::tracker::get_update_tracker)
            .service(super::route::tracker::post_update_tracker)
            .service(super::route::tracker::get_delete_tracker)
            .service(super::route::tracker::get_track_impression)
    })
    .backlog(4096)
    .shutdown_timeout(5);

    if config.worker_count != 0 {
        http_server = http_server.workers(config.worker_count as usize);
    }

    match match &config.bind_address {
        SocketListener::Tcp(address) => http_server.bind(address),
        #[cfg(unix)]
        SocketListener::Unix(path) => http_server.bind_uds(path),
    } {
        Ok(server_socket) => {
            log::info!("Listening on {:?}", &config.bind_address);
            server_socket
        }
        Err(err) => {
            log::error!("Couldn't bind to '{:?}'", &config.bind_address);
            panic!("{:?}", err);
        }
    }
    .run()
    .await
    .map_err(HttpServeError::from)
}
