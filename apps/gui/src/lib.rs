#![warn(missing_docs)]

pub mod cmd;
pub mod log;
pub mod macros;
pub mod run;

pub extern crate api;
pub extern crate macros as whmacros;
pub extern crate specta;
pub extern crate tauri;
pub extern crate tauri_specta;
pub extern crate tokio;
pub extern crate whcore;

#[macro_use]
extern crate tracing;

use api::install::progress::ProgressPayload;
use data::diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use whcore::state::TState;

pub type AppState<'a> = TState<'a, Pool<ConnectionManager<SqliteConnection>>>;

#[macro_export]
macro_rules! commands {
    ($($map: ident),* $(,)?; $($command: path),* $(,)?; $($event: ident),* $(,)?) => {
        $crate::whmacros::serde_funcs![$($command),*];

        pub fn funcs() -> $crate::specta::functions::CollectFunctionsResult {
            let map = $crate::whcore::merge_type_maps(vec![$($map::type_map()),*]);

            $crate::specta::functions::collect_functions![map; $($command),*]
        }

        pub fn invoker<R: $crate::tauri::Runtime>() -> Box<dyn Fn($crate::tauri::Invoke<R>) + Send + Sync + 'static> {
            Box::new($crate::tauri::generate_handler![$($command),*])
        }

        pub fn cmds<R: $crate::tauri::Runtime>() -> (
            $crate::specta::functions::CollectFunctionsResult,
            Box<dyn Fn($crate::tauri::Invoke<R>) + Send + Sync + 'static>,
        ) {
            (funcs(), invoker())
        }

        pub fn events<R: $crate::tauri::Runtime>() -> ($crate::tauri_specta::EventCollection, Vec<$crate::tauri_specta::EventDataType>, $crate::specta::TypeMap) {
            $crate::tauri_specta::collect_events![$($event),*]
        }

        pub fn ctx() -> $crate::tauri::Context<$crate::tauri::utils::assets::EmbeddedAssets> {
            $crate::tauri::generate_context!()
        }
    }
}

commands![
    data, api, whcore, mcmeta;

    cmd::base::get_plugins,
    cmd::base::get_dirs,
    cmd::base::get_source_id,
    cmd::instance::get_instances,
    cmd::instance::get_instance,
    cmd::instance::add_instance,
    cmd::instance::delete_instance,
    cmd::instance::create_instance,
    cmd::instance::update_instance,
    cmd::plugin::info,
    cmd::plugin::search_mods,
    cmd::plugin::get_mod,
    cmd::plugin::get_mod_versions,
    cmd::plugin::get_mod_version,
    cmd::plugin::get_download_url,
    cmd::plugin::launch_game,
    cmd::plugin::sources,
    cmd::plugin::get_latest_version,
    cmd::mods::install_mod,
    cmd::mods::uninstall_mod,
    cmd::mods::get_mods,
    cmd::loader::get_loaders,
    cmd::loader::get_latest_loader,
    cmd::loader::install_loader;

    ProgressPayload
];
