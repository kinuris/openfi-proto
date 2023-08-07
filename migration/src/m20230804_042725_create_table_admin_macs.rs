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
                    .table(AdminMac::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminMac::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminMac::Mac).string().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(AdminMac::Table)
            .columns([AdminMac::Mac])
            .values_panic(["7e:c6:be:e4:b4:97".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(AdminMac::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AdminMac {
    Table,
    Id,
    Mac,
}
