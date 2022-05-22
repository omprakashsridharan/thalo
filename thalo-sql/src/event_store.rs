use crate::migration::{Migrator, MigratorTrait};
use async_trait::async_trait;
use sea_orm::sea_query::Expr;
use sea_orm::{
    error::DbErr, ColumnTrait, ConnectOptions, Database, DatabaseConnection, QueryFilter, Set,
};
use sea_orm::{Condition, EntityTrait, FromQueryResult, QuerySelect, TransactionTrait};
use serde::{de::DeserializeOwned, Serialize};
use thalo::event::EventType;

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

#[derive(FromQueryResult)]
struct MaxSequence {
    sequence: u64,
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
        id: &<A as Aggregate>::ID,
    ) -> Result<Option<u64>, Self::Error>
    where
        A: Aggregate,
    {
        let max_sequences = event::Entity::find()
            .column_as(event::Column::Id.max(), "sequence_max")
            .filter(event::Column::AggregateType.eq(id.to_string()))
            .into_model::<MaxSequence>()
            .all(&self.db)
            .await
            .unwrap();
        if max_sequences.len() > 0 {
            return Ok(Some(max_sequences.get(0).unwrap().sequence));
        }
        Ok(None)
    }

    async fn save_events<A>(
        &self,
        id: &<A as Aggregate>::ID,
        events: &[<A as Aggregate>::Event],
    ) -> Result<Vec<u64>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: Serialize,
    {
        let sequence = self.load_aggregate_sequence::<A>(id).await?.unwrap_or(0);
        let txn = (&self.db).begin().await?;

        let mut event_models_to_be_save: Vec<event::ActiveModel> = vec![];
        for (index, evt) in events.iter().enumerate() {
            event_models_to_be_save.push(event::ActiveModel {
                aggregate_type: Set(<A as TypeId>::type_id().to_string()),
                aggregate_id: Set(id.to_string()),
                sequence: Set(sequence + index as u64 + 1),
                event_type: Set(String::from(evt.event_type())),
                event_data: Set(serde_json::to_value(evt).map_err(Error::SerializeEvent)?),
                ..Default::default()
            })
        }
        let res = event::Entity::insert_many(event_models_to_be_save)
            .exec(&txn)
            .await?;
        txn.commit().await?;
        Ok(vec![res.last_insert_id])
    }
}
