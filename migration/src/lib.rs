pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240822_161852_create_friendlist;
mod m20240822_171309_add_field_to_friendlist;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240822_161852_create_friendlist::Migration),
            Box::new(m20240822_171309_add_field_to_friendlist::Migration),
        ]
    }
}
