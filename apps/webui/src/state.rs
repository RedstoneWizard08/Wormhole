use std::sync::Arc;

use anyhow::Result;
use api::{
    plugin::Plugin,
    plugins::{Kerbal1Plugin, Kerbal2Plugin, MinecraftPlugin},
    tauri::TauriPlugin,
};
use data::{
    diesel::r2d2::{ConnectionManager, Pool},
    Conn,
};
use tauri::{api::ipc::CallbackFn, AppHandle, Builder, InvokeResponse, Window, Wry};
use wormhole_gui::{ctx, invoker};

use crate::{cli::Cli, route::CHANNELS};

#[derive(Clone)]
pub struct AppState {
    pub options: Cli,
    pub pool: Pool<ConnectionManager<Conn>>,
    pub app: Arc<AppHandle>,
}

impl AppState {
    pub async fn new(
        options: Cli,
        pool: Pool<ConnectionManager<Conn>>,
        init_script: String,
    ) -> Result<Self> {
        let app = Builder::default()
            .invoke_system(init_script, Self::respond)
            .plugin(TauriPlugin::new(Kerbal1Plugin::new())?)
            .plugin(TauriPlugin::new(Kerbal2Plugin::new())?)
            .plugin(TauriPlugin::new(MinecraftPlugin::new())?)
            .manage(pool.clone())
            .manage(invoker::<Wry>())
            .build(ctx())?;

        Ok(Self {
            options,
            pool,
            app: Arc::new(app.handle()),
        })
    }

    pub fn respond(_window: Window, resp: InvokeResponse, callback: CallbackFn, _err: CallbackFn) {
        let key = callback.0;
        let channels = CHANNELS.lock().unwrap();
        let tx = channels.get(&key).unwrap();

        tx.send(resp).unwrap();
    }
}
