use super::{Delta, Event, State, TimedEvent, ConvertError};
use crate::io::provider::{Timestamp, StreamState, StreamDelta};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    pub timestamp: Option<Timestamp>,
    pub value: f64,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            timestamp: None,
            value: 0.0,
        }
    }
}

impl TryFrom<StreamState> for CounterState {
    type Error = ConvertError;

    fn try_from(state: StreamState) -> Result<Self, ConvertError> {
        match state {
            StreamState::Counter(state) => Ok(state),
            _ => Err(ConvertError),
        }
    }
}

impl State for CounterState {
    type Delta = CounterDelta;

    fn apply(&mut self, delta: Self::Delta) {
        self.timestamp = Some(delta.timestamp);
        self.value += delta.delta;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterDelta {
    timestamp: Timestamp,
    delta: f64,
}

impl TryFrom<StreamDelta> for CounterDelta {
    type Error = ConvertError;

    fn try_from(delta: StreamDelta) -> Result<Self, ConvertError> {
        match delta {
            StreamDelta::Counter(delta) => Ok(delta),
            _ => Err(ConvertError),
        }
    }
}

impl Delta for CounterDelta {
    type Event = CounterEvent;

    fn produce(event: TimedEvent<Self::Event>) -> Self {
        let delta;
        match event.event {
            CounterEvent::Increment(value) => {
                delta = value;
            }
        }
        Self {
            timestamp: event.timestamp,
            delta,
        }
    }

    fn combine(&mut self, event: TimedEvent<Self::Event>) {
        self.timestamp = event.timestamp;
        match event.event {
            CounterEvent::Increment(value) => {
                self.delta += value;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CounterEvent {
    Increment(f64),
}

impl Event for CounterEvent {
    type State = CounterState;
    type Delta = CounterDelta;
}
