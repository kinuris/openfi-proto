use sea_orm_migration::prelude::*;

use crate::{
    m20230803_063215_create_table_plans::Plan, m20230803_063339_create_table_data_plans::DataPlan,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let insert = Query::insert()
            .into_table(Plan::Table)
            .columns([Plan::Name, Plan::SecondsGiven, Plan::CreditCost])
            .values_panic(["10mins".into(), 600.into(), 1.into()])
            .values_panic(["1hr".into(), 3600.into(), 5.into()])
            .values_panic(["6hrs".into(), (3600 * 6).into(), 15.into()])
            .values_panic(["1D".into(), (3600 * 24).into(), 30.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(DataPlan::Table)
            .columns([
                DataPlan::Name,
                DataPlan::MegabytesGiven,
                DataPlan::CreditCost,
            ])
            .values_panic(["100MB".into(), 200.into(), 1.into()])
            .values_panic(["1GB".into(), 1000.into(), 5.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Add delete statement

        todo!("");
    }
}
