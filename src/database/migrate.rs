use migration::MigratorTrait;

use crate::model::config::CliConfig;

pub enum MigrationAction {
    Check(CliConfig),
    Run(CliConfig),
}

pub async fn process_database_migrate(
    action: MigrationAction,
) -> Result<(), super::connection::ConnectError> {
    match action {
        MigrationAction::Check(config) => {
            let db =
                super::connection::get_database_connection(&config.database_url, config.log_level)
                    .await?;

            migration::Migrator::status(&db)
                .await
                .map_err(super::connection::ConnectError::from)
        }
        MigrationAction::Run(config) => {
            let db =
                super::connection::get_database_connection(&config.database_url, config.log_level)
                    .await?;

            migration::Migrator::up(&db, None)
                .await
                .map_err(super::connection::ConnectError::from)
        }
    }
}
