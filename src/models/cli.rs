const ABOUT_WITH_LICENSE: &str = "A simple web based image pixel tracker.

This program is free software: you can redistribute it and/or modify it under the terms \
of the GNU Affero General Public License as published by the Free Software Foundation, \
either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; \
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

See the GNU Affero General Public License for more details:
<https://www.gnu.org/licenses/agpl-3.0.txt>

This product includes software developed by the OpenSSL Project for use in \
the OpenSSL Toolkit. <https://www.openssl.org/>";

/// A simple web based image pixel tracker.
/// This program comes with ABSOLUTELY NO WARRANTY;
/// This is free software, and you are welcome to redistribute it under certain conditions;
/// type `--help` for more details.
#[derive(clap::Parser, Debug)]
#[clap(version, about, long_about = Some(ABOUT_WITH_LICENSE))]
pub struct Cli {
    /// Database URL to connect to and store user / tracking data.
    #[clap(short, long, env = "DATABASE_URL")]
    pub database_url: String,
    /// <IPv4 / IPv6>:port or socket to listen on.
    #[clap(short, long, env = "LISTEN_HANDLE")]
    pub listen: String,
    /// Log level to use. Keep in mind that this can include PII.
    /// Possible values include: "off", "error", "warn", "info", "debug", "trace".
    #[clap(short = 'v', long, env = "LOG_LEVEL", default_value_t = log::LevelFilter::Warn)]
    pub log_level: log::LevelFilter,
    /// Worker thread count for handling incoming HTTP requests.
    #[clap(short = 'w', long, env = "WORKER_COUNT", default_value_t = 0)]
    pub worker_count: u8,
}

#[cfg(test)]
mod tests {
    use super::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;

        Cli::command().debug_assert()
    }
}
