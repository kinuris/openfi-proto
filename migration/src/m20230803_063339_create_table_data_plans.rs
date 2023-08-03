use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(DataPlan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataPlan::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DataPlan::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(DataPlan::CreditCost).unsigned().not_null())
                    .col(
                        ColumnDef::new(DataPlan::MegabytesGiven)
                            .unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DataPlan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DataPlan {
    Table,
    Id,
    Name,
    CreditCost,
    MegabytesGiven,
}
