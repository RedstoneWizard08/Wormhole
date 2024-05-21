pub mod macros;

pub extern crate api;
pub extern crate macros as whmacros;
pub extern crate specta;
pub extern crate tauri;
pub extern crate tokio;
pub extern crate whcore;

use anyhow::Result;

use api::{plugin::PluginInfo, register::PLUGINS};
use data::{
    diesel::{
        delete, insert_into,
        r2d2::{ConnectionManager, Pool},
        ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
    },
    instance::Instance,
    schema::instances::dsl::{game_id as gid, id, instances},
    source::SourceMapping,
};

use query::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions},
};
use specta::functions::CollectFunctionsResult;
use tauri::{utils::assets::EmbeddedAssets, Context, Invoke, Runtime};
use whcore::{dirs::Dirs, manager::CoreManager, merge_type_maps, state::TState, Boolify};

pub type AppState<'a> = TState<'a, Pool<ConnectionManager<SqliteConnection>>>;

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_instances(game_id: i32, pool: AppState<'_>) -> Result<Vec<Instance>, bool> {
    let mut db = pool.get().bool()?;

    let items = instances
        .filter(gid.eq(game_id))
        .select(Instance::as_select())
        .load(&mut db)
        .bool()?;

    Ok(items)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn delete_instance(instance_id: i32, pool: AppState<'_>) -> Result<(), bool> {
    let mut db = pool.get().bool()?;

    delete(instances.filter(id.eq(instance_id)))
        .execute(&mut db)
        .bool()?;

    Ok(())
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn add_instance(instance: Instance, pool: AppState<'_>) -> Result<Instance, bool> {
    Ok(insert_into(instances)
        .values(instance)
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_instance(instance_id: i32, pool: AppState<'_>) -> Result<Instance, bool> {
    Ok(instances
        .filter(id.eq(instance_id))
        .select(Instance::as_select())
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_plugins(_pool: AppState<'_>) -> Result<Vec<PluginInfo>, bool> {
    let mut res = Vec::new();
    let lock = PLUGINS.lock().await;

    for plugin in lock.values() {
        res.push(plugin.as_info().await.ok_or(false)?);
    }

    Ok(res)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_dirs(_pool: AppState<'_>) -> Result<Dirs, bool> {
    Ok(CoreManager::get().dirs())
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_source_id(sid: i32, _pool: AppState<'_>) -> Result<String, bool> {
    Ok(SourceMapping::from(sid).as_str().to_string())
}

plugin_fn_proxy!(async info => info: () -> [opt] PluginInfo);
plugin_fn_proxy!(async search_mods => search_mods: (resolver: SourceMapping, query: Option<String>, opts: Option<QueryOptions>) -> [opt] Paginated<Mod>);
plugin_fn_proxy!(async get_mod => get_mod: (resolver: SourceMapping, mid: String) -> [opt] Mod);
plugin_fn_proxy!(async get_mod_versions => get_mod_versions: (resolver: SourceMapping, mid: String) -> [opt] Vec<ModVersion>);
plugin_fn_proxy!(async get_mod_version => get_mod_version: (resolver: SourceMapping, mid: String, version: String) -> [opt] ModVersion);
plugin_fn_proxy!(async get_download_url => get_download_url: (resolver: SourceMapping, project: String, version: Option<String>) -> [opt] String);
plugin_fn_proxy!(async launch_game => launch_game: (instance: Instance) -> ());
plugin_fn_proxy!(async sources => sources: () -> [opt] Vec<String>);

#[macro_export]
macro_rules! funcs {
    ($ns: ident::$fn: ident) => {
        $ns::$fn![
            get_instances,
            get_instance,
            get_plugins,
            add_instance,
            delete_instance,
            get_dirs,
            info,
            search_mods,
            get_mod,
            get_mod_versions,
            get_mod_version,
            get_download_url,
            launch_game,
            sources,
            get_source_id
        ]
    };

    ($ns: ident::$fn: ident;) => {
        $ns::$fn![
            get_instances,
            get_instance,
            get_plugins,
            add_instance,
            delete_instance,
            get_dirs,
            info,
            search_mods,
            get_mod,
            get_mod_versions,
            get_mod_version,
            get_download_url,
            launch_game,
            sources,
            get_source_id
        ];
    };
}

funcs!(whmacros::serde_funcs;);

pub fn funcs() -> CollectFunctionsResult {
    let map = merge_type_maps(vec![data::type_map(), api::type_map(), whcore::type_map()]);

    specta::functions::collect_functions![
        map;
        get_instances,
        get_instance,
        get_plugins,
        add_instance,
        delete_instance,
        get_dirs,
        info,
        search_mods,
        get_mod,
        get_mod_versions,
        get_mod_version,
        get_download_url,
        launch_game,
        sources,
        get_source_id
    ]
}

pub fn invoker<R: Runtime>() -> Box<dyn Fn(Invoke<R>) + Send + Sync + 'static> {
    Box::new(funcs!(tauri::generate_handler))
}

pub fn cmds<R: Runtime>() -> (
    CollectFunctionsResult,
    Box<dyn Fn(Invoke<R>) + Send + Sync + 'static>,
) {
    (funcs(), invoker())
}

pub fn ctx() -> Context<EmbeddedAssets> {
    tauri::generate_context!()
}
