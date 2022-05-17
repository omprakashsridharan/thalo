use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{
    any::{Any, AnyConnectOptions},
    error::Error,
    pool::Pool,
    types::chrono::{DateTime, FixedOffset},
    Decode, FromRow,
};
use thalo::{
    aggregate::{Aggregate, TypeId},
    event::AggregateEventEnvelope,
    event_store::EventStore,
};

const LOAD_EVENTS_QUERY: &str = include_str!("queries/load_events.sql");

#[derive(Clone)]
pub struct SqlEventStore {
    pool: Pool<Any>,
}

impl SqlEventStore {
    pub async fn connect(connect_options: AnyConnectOptions) -> Result<Self, Error> {
        let x = Pool::connect_with(connect_options).await;
        match x {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow, Decode)]
struct LoadEventsRow {
    pub id: i64,
    #[sqlx]
    pub created_at: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub sequence: i64,
    pub event_data: String,
}

#[async_trait]
impl EventStore for SqlEventStore {
    type Error = i32;

    async fn load_events<A>(
        &self,
        id: Option<&<A as Aggregate>::ID>,
    ) -> Result<Vec<AggregateEventEnvelope<A>>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: DeserializeOwned,
    {
        let rows: Vec<LoadEventsRow> = sqlx::query_as(LOAD_EVENTS_QUERY)
            .bind(&<A as TypeId>::type_id())
            .bind(&id.map(|id| id.to_string()))
            .fetch_all(&self.pool)
            .await
            .unwrap();
        println!("{:?}", rows);
        todo!()
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
