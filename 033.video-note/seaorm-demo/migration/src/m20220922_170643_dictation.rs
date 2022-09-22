use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dictation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dictation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dictation::Name).string())
                    .col(ColumnDef::new(Dictation::StartAt).string())
                    .col(ColumnDef::new(Dictation::EndAt).string())
                    .col(ColumnDef::new(Dictation::Total).integer())
                    .col(ColumnDef::new(Dictation::Wrong).integer())
                    .col(ColumnDef::new(Dictation::Correct).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Dictation::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Dictation {
    Table,
    Id,
    Name,
    StartAt,
    EndAt,
    Total,
    Wrong,
    Correct,
}
