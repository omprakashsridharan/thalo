use sea_orm_migration::prelude::*;
use crate::entity::event::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_create_events_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
        .table(Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(Column::Id)
            .unsigned()
            .not_null()
            .auto_increment()
            .primary_key(),
        )
        .col(ColumnDef::new(Column::AggregateId).string().not_null())
        .col(ColumnDef::new(Column::AggregateType).string())
        .col(ColumnDef::new(Column::CreatedAt).date_time().not_null())
        .col(ColumnDef::new(Column::EventData).json().not_null())
        .col(ColumnDef::new(Column::Sequence).unsigned().not_null())
        .to_owned()
    ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
