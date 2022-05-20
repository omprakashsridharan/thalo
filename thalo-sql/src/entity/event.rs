use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: DateTimeWithTimeZone,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub sequence: u64,
    pub event_data: Json
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
