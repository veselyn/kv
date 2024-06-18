use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Key::Table)
                    .col(ColumnDef::new(Key::Id).text().not_null().primary_key())
                    .col(
                        ColumnDef::new(Key::Type)
                            .text()
                            .not_null()
                            .check(Expr::col(Key::Type).is_in(["json"])),
                    )
                    .col(ColumnDef::new(Key::Value).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Key::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Key {
    Table,
    Id,
    Type,
    Value,
}
