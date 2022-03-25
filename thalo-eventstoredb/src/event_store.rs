use std::{
    fs::{self, File},
    io::{self, BufRead, Write},
    path::Path,
    sync::Mutex,
};

use async_trait::async_trait;
use eventstore::Client;
use serde::{de::DeserializeOwned, Serialize};
use thalo::{aggregate::Aggregate, event::AggregateEventEnvelope, event_store::EventStore};

#[derive(Debug)]
pub struct EventStoreDB {
    client: Client,
}

impl EventStoreDB {
    pub fn new(client: Client) -> Self {
        EventStoreDB { client }
    }
}

#[async_trait]
impl EventStore for EventStoreDB {
    type Error = Error;

    async fn load_events<A>(
        &self,
        id: Option<&<A as Aggregate>::ID>,
    ) -> Result<Vec<AggregateEventEnvelope<A>>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: DeserializeOwned,
    {
    }

    async fn load_events_by_id<A>(
        &self,
        ids: &[usize],
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
    ) -> Result<Option<usize>, Self::Error>
    where
        A: Aggregate,
    {
        todo!()
    }

    async fn save_events<A>(
        &self,
        id: &<A as Aggregate>::ID,
        events: &[<A as Aggregate>::Event],
    ) -> Result<Vec<usize>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: Serialize,
    {
        todo!()
    }
}
