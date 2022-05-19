use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection, ConnectOptions, error::DbErr, QueryFilter, ColumnTrait};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sea_orm::EntityTrait;

use crate::{entities::event::{self, Model as EventModel,Entity as EventEntity}, Error};

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
        .filter(event::Column::AggregateId.eq(<A as TypeId>::type_id()))
        .filter(event::Column::AggregateType.eq(id.map(|id| id.to_string())))
        .all(&self.db)    
        .await.unwrap();

        Ok(events.into_iter().map(|row| {
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
        .collect::<Result<Vec<_>, _>>()?
    )
    }

    async fn load_events_by_id<A>(
        &self,
        ids: &[u64],
    ) -> Result<Vec<AggregateEventEnvelope<A>>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: DeserializeOwned,
    {
        todo!()
    }

    async fn load_aggregate_sequence<A>(
        &self,
        id: &<A as Aggregate>::ID,
    ) -> Result<Option<u64>, Self::Error>
    where
        A: Aggregate,
    {
        todo!()
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
        todo!()
    }
}
