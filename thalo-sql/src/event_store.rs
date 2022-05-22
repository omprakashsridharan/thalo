use crate::migration::{Migrator, MigratorTrait};
use async_trait::async_trait;
use sea_orm::sea_query::Expr;
use sea_orm::{
    error::DbErr, ColumnTrait, ConnectOptions, Database, DatabaseConnection, QueryFilter,
};
use sea_orm::{Condition, EntityTrait};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    entity::event::{self, Entity as EventEntity, Model as EventModel},
    Error,
};

use thalo::{
    aggregate::{Aggregate, TypeId},
    event::AggregateEventEnvelope,
    event_store::EventStore,
};

#[derive(Clone)]
pub struct SqlEventStore {
    db: DatabaseConnection,
}

impl SqlEventStore {
    pub async fn connect(connect_options: ConnectOptions) -> Result<Self, DbErr> {
        let db = Database::connect(connect_options).await?;
        Migrator::up(&db, None).await?;
        Ok(Self { db })
    }
}

#[async_trait]
impl EventStore for SqlEventStore {
    type Error = Error;

    async fn load_events<A>(
        &self,
        id: Option<&<A as Aggregate>::ID>,
    ) -> Result<Vec<AggregateEventEnvelope<A>>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: DeserializeOwned,
    {
        let events: Vec<EventModel> = EventEntity::find()
            .filter(
                Condition::any()
                    .add(event::Column::AggregateId.eq(<A as TypeId>::type_id()))
                    .add(Expr::col(event::Column::AggregateId).is_null()),
            )
            .filter(event::Column::AggregateType.eq(id.map(|id| id.to_string())))
            .all(&self.db)
            .await
            .unwrap();

        Ok(events
            .into_iter()
            .map(|row| {
                let event = serde_json::from_value(row.event_data)
                    .map_err(|err| Error::DeserializeDbEvent(row.id, err))?;
                Result::<_, Self::Error>::Ok(AggregateEventEnvelope::<A> {
                    id: row.id,
                    created_at: row.created_at,
                    aggregate_type: row.aggregate_type,
                    aggregate_id: row.aggregate_id,
                    sequence: row.sequence,
                    event,
                })
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    async fn load_events_by_id<A>(
        &self,
        ids: &[u64],
    ) -> Result<Vec<AggregateEventEnvelope<A>>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: DeserializeOwned,
    {
        let events: Vec<EventModel> = EventEntity::find()
            .filter(Expr::col(event::Column::AggregateId).is_in(Vec::from(ids)))
            .all(&self.db)
            .await
            .unwrap();

        Ok(events
            .into_iter()
            .map(|row| {
                let event = serde_json::from_value(row.event_data)
                    .map_err(|err| Error::DeserializeDbEvent(row.id, err))?;
                Result::<_, Self::Error>::Ok(AggregateEventEnvelope::<A> {
                    id: row.id,
                    created_at: row.created_at,
                    aggregate_type: row.aggregate_type,
                    aggregate_id: row.aggregate_id,
                    sequence: row.sequence,
                    event,
                })
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    async fn load_aggregate_sequence<A>(
        &self,
        _id: &<A as Aggregate>::ID,
    ) -> Result<Option<u64>, Self::Error>
    where
        A: Aggregate,
    {
        todo!()
    }

    async fn save_events<A>(
        &self,
        _id: &<A as Aggregate>::ID,
        _events: &[<A as Aggregate>::Event],
    ) -> Result<Vec<u64>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: Serialize,
    {
        todo!()
    }
}
