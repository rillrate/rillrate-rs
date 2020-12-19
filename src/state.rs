use crate::protocol::StreamType;
use crate::providers::provider::{DataReceiver, Joint};
use futures::channel::mpsc;
use meio::prelude::Action;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::watch;

pub(crate) static RILL_STATE: OnceCell<RillState> = OnceCell::new();

pub(crate) enum ControlEvent {
    RegisterProvider {
        joint: Arc<Joint>,
        stream_type: StreamType,
        active: watch::Sender<bool>,
        rx: DataReceiver,
    },
}

impl Action for ControlEvent {}

pub(crate) type ControlSender = mpsc::UnboundedSender<ControlEvent>;
pub(crate) type ControlReceiver = mpsc::UnboundedReceiver<ControlEvent>;

pub(crate) struct RillState {
    sender: ControlSender,
}

impl RillState {
    pub fn create() -> (ControlReceiver, Self) {
        let (tx, rx) = mpsc::unbounded();
        let this = Self { sender: tx };
        (rx, this)
    }

    pub fn send(&self, event: ControlEvent) {
        self.sender
            .unbounded_send(event)
            .expect("rill actors not started");
    }
}
