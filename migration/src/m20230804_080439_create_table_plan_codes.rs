use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table = Statement::from_string(
            DbBackend::Sqlite,
            "
            CREATE TABLE redeemable_code (
                id INTEGER NOT NULL PRIMARY KEY,
                code CHAR(10) NOT NULL,
                kind TEXT CHECK(kind IN ('DATA', 'CREDIT', 'TIME')) NOT NULL,
                units INT NOT NULL
            );
        ",
        );

        manager
            .get_connection()
            .execute(create_table)
            .await
            .unwrap();
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_table = Statement::from_string(
            DbBackend::Sqlite,
            "
            DROP TABLE IF EXISTS redeemable_code;
        ",
        );

        manager.get_connection().execute(drop_table).await.unwrap();
        Ok(())
    }
}
