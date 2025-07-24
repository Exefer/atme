use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_charge_records_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChargeRecords::Table)
                    .if_not_exists()
                    .col(pk_auto(ChargeRecords::Id))
                    .col(
                        timestamp(ChargeRecords::StartTimestamp)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(timestamp_null(ChargeRecords::EndTimestamp))
                    .col(integer(ChargeRecords::StartPercentage).not_null())
                    .col(integer_null(ChargeRecords::EndPercentage))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChargeRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChargeRecords {
    Table,
    Id,
    StartTimestamp,
    EndTimestamp,
    StartPercentage,
    EndPercentage,
}
