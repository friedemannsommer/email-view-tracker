pub use sea_orm_migration::prelude::*;

mod m20221127_000001_create_users;
mod m20221127_000002_create_trackers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221127_000001_create_users::Migration),
            Box::new(m20221127_000002_create_trackers::Migration),
        ]
    }
}
