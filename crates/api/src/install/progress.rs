use std::{future::Future, pin::Pin};

use tauri::Manager;
use tauri_specta::Event as TEvent;

use crate::{EVENT_BUS, TAURI_HANDLE};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Type
)]
pub struct ProgressPayload {
    /// The total number of iterations. For a request or
    /// file operation, this will be the total size of
    /// the file, in bytes.
    pub total: u64,

    /// The current processed iterations count. For a
    /// request or file operation, this will be how many
    /// bytes have been received/processed/written.
    pub current: u64,

    /// The name of the file or operation being done.
    pub name: String,
}

impl TEvent for ProgressPayload {
    const NAME: &'static str = "progress_callback";
}

pub fn tauri_progress(
    total: u64,
    current: u64,
    name: String,
) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>> {
    Box::pin(async move {
        let payload = ProgressPayload {
            total,
            current,
            name,
        };

        let lock = TAURI_HANDLE.lock().await;

        if let Some(handle) = lock.clone() {
            handle.emit_all("progress_callback", payload).unwrap();
        } else {
            EVENT_BUS.0.send(payload).unwrap();
        }
    })
}
