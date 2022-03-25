//! Event store

use async_trait::async_trait;
use futures_util::{future::BoxFuture, stream::BoxStream};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    aggregate::Aggregate,
    event::{AggregateEventEnvelope, EventEnvelope, IntoEvents},
};

/// Used to store & load events.
///
/// Trait inspired by [eventually::EventStore](https://docs.rs/eventually/latest/eventually/trait.EventStore.html).
pub trait EventStore {
    /// The aggregate ID.
    type AggregateId;

    /// Event type before stored.
    type Event;

    /// Stored event type.
    type PersistedEvent;

    /// The error type.
    type Error;

    /// Append events to the event store.
    fn append(
        &mut self,
        aggregate_id: Self::AggregateId,
        events: &[Self::Event],
    ) -> BoxFuture<Result<Vec<u64>, Self::Error>>;

    /// Stream events from the event store.
    fn stream(
        &self,
        aggregate_id: Self::AggregateId,
        select: Select,
    ) -> BoxStream<Result<Self::PersistedEvent, Self::Error>>;

    /// Stream all events from the event store.
    fn stream_all(&self, select: Select) -> BoxStream<Result<Self::PersistedEvent, Self::Error>>;
}

/// Selection operation for the events to capture in an [`EventStream`].
///
/// [`EventStream`]: type.EventStream.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Select {
    /// To return all the [`Event`]s in the [`EventStream`].
    ///
    /// [`Event`]: trait.EventStore.html#associatedtype.Event
    /// [`EventStream`]: type.EventStream.html
    All,

    /// To return a slice of the [`EventStream`], starting from
    /// those [`Event`]s with version **greater or equal** than
    /// the one specified in this variant.
    ///
    /// [`Event`]: trait.EventStore.html#associatedtype.Event
    /// [`EventStream`]: type.EventStream.html
    From(u64),
}
