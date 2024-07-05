use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{
    plugin::{Plugin, Result},
    AppHandle, Manager, Runtime,
};
use tokio::sync::mpsc::unbounded_channel;

use crate::util::TripleS;

use super::{router::Router, Method};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriCommandInput {
    pub command: String,
    pub method: Method,
    pub data: Value,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriCommandOutput {
    pub command: String,
    pub method: Method,
    pub result: Value,
    pub id: String,
}

pub struct RouterPlugin<Cx: TripleS + Clone> {
    pub(crate) router: Router<Cx>,
    pub(crate) state: Cx,
}

impl<R: Runtime, Cx: TripleS + Clone> Plugin<R> for RouterPlugin<Cx> {
    fn name(&self) -> &'static str {
        "rpc-rs"
    }

    fn initialize(&mut self, app: &AppHandle<R>, _config: Value) -> Result<()> {
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
}

impl<Cx: TripleS + Clone> Router<Cx> {
    pub fn tauri(self, cx: Cx) -> RouterPlugin<Cx> {
        RouterPlugin {
            router: self,
            state: cx,
        }
    }
}
