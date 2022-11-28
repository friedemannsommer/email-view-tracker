use std::ffi::OsStr;
use std::io::Write;

#[derive(Clone)]
pub struct SocketListenerParser;

#[derive(Clone)]
pub struct StringWithLength(LengthComparison);

#[derive(Debug, Clone)]
pub enum SocketListener {
    Tcp(std::net::SocketAddr),
    #[cfg(unix)]
    Unix(std::path::PathBuf),
}

#[derive(Debug)]
pub enum CliCommand {
    HttpServer(super::config::ServerConfig),
    MigrateCheck(super::config::CliConfig),
    MigrateRun(super::config::CliConfig),
}

#[derive(Clone)]
enum LengthComparison {
    Equal(usize),
    GreaterOrEqual(usize),
}

const HTTP_START: &str = "start";
const MIGRATE: &str = "migrate";
const MIGRATE_RUN: &str = "run";
const MIGRATE_CHECK: &str = "check";
const LOG_LEVEL_HELP: &str = "Log level to use. Keep in mind that this can include PII.";

pub fn process_cli() -> Option<CliCommand> {
    let matches = get_command().get_matches();

    match matches.subcommand() {
        Some((HTTP_START, sub_matches)) => {
            return Some(CliCommand::HttpServer(parse_server_config(sub_matches)))
        }
        Some((MIGRATE, migrate_matches)) => match migrate_matches.subcommand() {
            Some((MIGRATE_CHECK, sub_matches)) => {
                return Some(CliCommand::MigrateCheck(parse_cli_config(sub_matches)))
            }
            Some((MIGRATE_RUN, sub_matches)) => {
                return Some(CliCommand::MigrateRun(parse_cli_config(sub_matches)))
            }
            _ => {}
        },
        _ => {}
    }

    None
}

fn get_command() -> clap::Command {
    let database_arg = clap::builder::Arg::new("database_url")
        .short('d')
        .long("database-url")
        .env("DATABASE_URL")
        .help("(MySQL / PostgreSQL) Database URL")
        .required(true)
        .value_parser(clap::builder::NonEmptyStringValueParser::new());
    let log_level_arg = clap::builder::Arg::new("log_level")
        .short('v')
        .long("log-level")
        .env("LOG_LEVEL")
        .help(LOG_LEVEL_HELP)
        .default_value("warn")
        .value_parser(clap::value_parser!(log::LevelFilter))
        .long_help(format!(
            "{}\n{}",
            LOG_LEVEL_HELP,
            r#"Possible values include: "off", "error", "warn", "info", "debug", "trace"."#
        ));

    clap::command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .after_help(r#"This program comes with ABSOLUTELY NO WARRANTY;
This is free software, and you are welcome to redistribute it under certain conditions;
Type `--help` for more details."#)
        .after_long_help(r#"This program is free software: you can redistribute it and/or modify it under the terms \
of the GNU Affero General Public License as published by the Free Software Foundation, \
either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; \
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

See the GNU Affero General Public License for more details:
<https://www.gnu.org/licenses/agpl-3.0.txt>"#)
        .subcommand(
            clap::Command::new(HTTP_START)
                .about("Start HTTP server")
                .arg(database_arg.clone())
                .arg(clap::builder::Arg::new("bind_address")
                    .short('l')
                    .long("listen")
                    .env("BIND_ADDRESS")
                    .help("(IPv4 / IPv6):port or socket to listen on.")
                    .required(true)
                    .value_parser(SocketListenerParser)
                )
                .arg(clap::builder::Arg::new("cookie_secret")
                    .short('c')
                    .long("cookie-secret")
                    .env("COOKIE_SECRET")
                    .help("Secret will be used to encrypt session cookie. Must contain at least 64 bytes.")
                    .required(true)
                    .value_parser(StringWithLength(LengthComparison::Equal(64)))
                )
                .arg(clap::builder::Arg::new("password_secret")
                    .short('p')
                    .long("password-secret")
                    .env("PASSWORD_SECRET")
                    .help("Secret will be used as salt for password hashing. Must contain at least 16 bytes.")
                    .required(true)
                    .value_parser(StringWithLength(LengthComparison::GreaterOrEqual(16)))
                )
                .arg(clap::builder::Arg::new("worker_count")
                    .short('w')
                    .long("worker-count")
                    .env("WORKER_COUNT")
                    .help("Worker thread count for handling incoming HTTP requests.")
                    .default_value("0")
                    .value_parser(clap::value_parser!(u8))
                )
                .arg(log_level_arg.clone())
                .arg_required_else_help(true),
        )
        .subcommand(
            clap::Command::new(MIGRATE)
                .about("Check or run migrations")
                .subcommand_required(true)
                .subcommand(
                    clap::Command::new(MIGRATE_RUN)
                        .arg(database_arg.clone())
                        .arg(log_level_arg.clone())
                        .arg_required_else_help(true)
                )
                .subcommand(
                    clap::Command::new(MIGRATE_CHECK)
                        .arg(database_arg)
                        .arg(log_level_arg)
                        .arg_required_else_help(true)
                )
        )
}

fn parse_server_config(matches: &clap::ArgMatches) -> super::config::ServerConfig {
    super::config::ServerConfig {
        bind_address: matches
            .get_one::<SocketListener>("bind_address")
            .unwrap()
            .clone(),
        cookie_secret: matches.get_one::<String>("cookie_secret").unwrap().clone(),
        database_url: matches.get_one::<String>("database_url").unwrap().clone(),
        log_level: *matches.get_one::<log::LevelFilter>("log_level").unwrap(),
        password_secret: matches
            .get_one::<String>("password_secret")
            .unwrap()
            .clone(),
        worker_count: *matches.get_one::<u8>("worker_count").unwrap(),
    }
}

fn parse_cli_config(matches: &clap::ArgMatches) -> super::config::CliConfig {
    super::config::CliConfig {
        database_url: matches.get_one::<String>("database_url").unwrap().clone(),
        log_level: *matches.get_one::<log::LevelFilter>("log_level").unwrap(),
    }
}

impl clap::builder::TypedValueParser for SocketListenerParser {
    type Value = SocketListener;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::error::Error> {
        use std::str::FromStr;

        let input = match value.to_str() {
            Some(val) => val.trim(),
            None => {
                return Err(clap::error::Error::new(
                    clap::error::ErrorKind::MissingRequiredArgument,
                )
                .with_cmd(cmd))
            }
        };

        if let Ok(address) = std::net::SocketAddr::from_str(input) {
            return Ok(SocketListener::Tcp(address));
        }

        #[cfg(unix)]
        if let Ok(socket_path) = std::path::PathBuf::from_str(input) {
            return Ok(SocketListener::Unix(socket_path));
        }

        Err(clap::error::Error::new(clap::error::ErrorKind::InvalidValue).with_cmd(cmd))
    }
}

impl clap::builder::TypedValueParser for StringWithLength {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::error::Error> {
        let input = match value.to_str() {
            Some(val) => val.trim(),
            None => {
                return Err(clap::error::Error::new(
                    clap::error::ErrorKind::MissingRequiredArgument,
                )
                .with_cmd(cmd))
            }
        };
        let input_length = input.len();

        if match self.0 {
            LengthComparison::Equal(length) => input_length != length,
            LengthComparison::GreaterOrEqual(length) => input_length < length,
        } {
            writeln!(
                std::io::stderr().lock(),
                "'{}' ({}) doesn't fulfill length requirement of {} characters",
                input,
                input_length,
                match self.0 {
                    LengthComparison::Equal(length) | LengthComparison::GreaterOrEqual(length) =>
                        length,
                },
            )
            .unwrap();

            return Err(
                clap::error::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd),
            );
        }

        Ok(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        super::get_command().debug_assert()
    }
}
