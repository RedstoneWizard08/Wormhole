//! Events module.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// An event.
pub trait Event: Serialize + DeserializeOwned {
    /// Get the name of this event.
    fn name() -> &'static str;
}

/// Create an event "bus" for sending/receiving [`Event`]s.
pub fn create_event_bus() -> (UnboundedSender<Value>, UnboundedReceiver<Value>) {
    unbounded_channel()
}
