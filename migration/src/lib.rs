pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230803_063215_create_table_plans;
mod m20230803_063339_create_table_data_plans;
mod m20230803_070833_plan_seed_data;
mod m20230804_042725_create_table_admin_macs;
mod m20230804_080439_create_table_plan_codes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230803_063215_create_table_plans::Migration),
            Box::new(m20230803_063339_create_table_data_plans::Migration),
            Box::new(m20230803_070833_plan_seed_data::Migration),
            Box::new(m20230804_042725_create_table_admin_macs::Migration),
            Box::new(m20230804_080439_create_table_plan_codes::Migration),
        ]
    }
}
