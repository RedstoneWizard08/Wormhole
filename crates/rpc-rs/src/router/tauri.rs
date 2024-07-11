//! The [`tauri`] support module.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{
    plugin::{Plugin, Result},
    AppHandle, Manager, Runtime,
};
use tokio::sync::mpsc::unbounded_channel;

use crate::util::TripleS;

use super::{router::Router, Method};

/// Command input model for [`tauri`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriCommandInput {
    /// The command that was run.
    pub command: String,

    /// The [`Method`].
    pub method: Method,

    /// The data provided.
    pub data: Value,

    /// An ID that represents this invocation.
    pub id: String,
}

/// Command output model for [`tauri`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriCommandOutput {
    /// The command that was run.
    pub command: String,

    /// The [`Method`].
    pub method: Method,

    /// The data returned from the invocation.
    pub result: Value,

    /// An ID that represents this invocation.
    pub id: String,
}

/// Invoker input model for [`tauri`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriInvokerInput {
    /// The command that was run.
    pub command: String,

    /// The data provided.
    pub data: Value,

    /// An ID that represents this invocation.
    pub id: String,
}

/// Invoker output model for [`tauri`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriInvokerOutput {
    /// The command that was run.
    pub command: String,

    /// The data returned from the invocation.
    pub result: Value,

    /// An ID that represents this invocation.
    pub id: String,
}

/// The [`Plugin`] for [`tauri`] which wraps the [`Router`] with its state.
pub struct RouterPlugin<Cx: TripleS + Clone> {
    /// The [`Router`].
    pub(crate) router: Router<Cx>,

    /// The state.
    pub(crate) state: Cx,
}

impl<Cx: TripleS + Clone> RouterPlugin<Cx> {
    fn initialize_modules<R: Runtime>(&mut self, app: &AppHandle<R>) -> Result<()> {
        let (tx, mut rx) = unbounded_channel::<TauriCommandInput>();
        let (rtx, mut rrx) = unbounded_channel::<TauriCommandOutput>();

        tokio::spawn({
            let state = self.state.clone();
            let router = self.router.clone();

            async move {
                while let Some(data) = rx.recv().await {
                    let rtx = rtx.clone();
                    let state = state.clone();
                    let router = router.clone();
                    let module = router
                        .modules
                        .iter()
                        .find(|v| v.0.clone() == data.command)
                        .map(|v| v.1.clone());

                    if let Some(module) = module.clone() {
                        tokio::spawn(async move {
                            let result = module
                                .exec(
                                    state,
                                    data.method,
                                    serde_json::to_string(&data.data).unwrap(),
                                )
                                .await;

                            if let Ok(result) =
                                serde_json::to_value(result.map_err(|v| v.to_string()))
                            {
                                let _ = rtx.send(TauriCommandOutput {
                                    command: data.command,
                                    id: data.id,
                                    method: data.method,
                                    result,
                                });
                            }
                        });
                    }
                }
            }
        });

        tokio::spawn({
            let handle = app.clone();

            async move {
                while let Some(data) = rrx.recv().await {
                    let _ = handle.emit_all("plugin:rpc-rs:transport:resp", data);
                }
            }
        });

        app.listen_global("plugin:rpc-rs:transport", move |event| {
            let data =
                serde_json::from_str::<TauriCommandInput>(event.payload().unwrap_or_default());

            let _ = match data {
                Ok(data) => tx.send(data),
                Err(_) => Ok(()),
            };
        });

        Ok(())
    }

    fn initialize_invokers<R: Runtime>(&mut self, app: &AppHandle<R>) -> Result<()> {
        let (tx, mut rx) = unbounded_channel::<TauriInvokerInput>();
        let (rtx, mut rrx) = unbounded_channel::<TauriInvokerOutput>();

        tokio::spawn({
            let state = self.state.clone();
            let router = self.router.clone();

            async move {
                while let Some(data) = rx.recv().await {
                    let rtx = rtx.clone();
                    let state = state.clone();
                    let router = router.clone();
                    let invoker = router
                        .invokers
                        .iter()
                        .find(|v| v.0.clone() == data.command)
                        .map(|v| v.1.clone());

                    if let Some(invoker) = invoker.clone() {
                        tokio::spawn(async move {
                            let result = invoker
                                .run(state, serde_json::to_string(&data.data).unwrap())
                                .await;

                            if let Ok(result) =
                                serde_json::to_value(result.map_err(|v| v.to_string()))
                            {
                                let _ = rtx.send(TauriInvokerOutput {
                                    command: data.command,
                                    id: data.id,
                                    result,
                                });
                            }
                        });
                    }
                }
            }
        });

        tokio::spawn({
            let handle = app.clone();

            async move {
                while let Some(data) = rrx.recv().await {
                    let _ = handle.emit_all("plugin:rpc-rs:transport:invoker:resp", data);
                }
            }
        });

        app.listen_global("plugin:rpc-rs:transport:invoker", move |event| {
            let data =
                serde_json::from_str::<TauriInvokerInput>(event.payload().unwrap_or_default());

            let _ = match data {
                Ok(data) => tx.send(data),
                Err(_) => Ok(()),
            };
        });

        Ok(())
    }
}

impl<R: Runtime, Cx: TripleS + Clone> Plugin<R> for RouterPlugin<Cx> {
    fn name(&self) -> &'static str {
        "rpc-rs"
    }

    fn initialize(&mut self, app: &AppHandle<R>, _config: Value) -> Result<()> {
        self.initialize_modules(app)?;
        self.initialize_invokers(app)?;

        Ok(())
    }
}

impl<Cx: TripleS + Clone> Router<Cx> {
    /// Convert this router into a [`RouterPlugin`], consuming it.
    pub fn tauri(self, cx: Cx) -> RouterPlugin<Cx> {
        RouterPlugin {
            router: self,
            state: cx,
        }
    }
}
