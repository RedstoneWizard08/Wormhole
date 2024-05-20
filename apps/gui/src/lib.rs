// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub mod macros;

pub extern crate api;
pub extern crate futures;
pub extern crate macros as whmacros;
pub extern crate specta;
pub extern crate tauri;
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

    for plugin in PLUGINS.lock().unwrap().values() {
        // For some reason, this doesn't implement Send or Sync,
        // even though the trait requires it and it's a bound on
        // the HashMap. So we have to use futures' block_on
        // function, which adds more dependencies to compile.
        // WTF, RUST

        res.push(futures::executor::block_on(plugin.as_info()));
    }

    Ok(res)
}

#[whmacros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_dirs(_pool: AppState<'_>) -> Result<Dirs, bool> {
    Ok(CoreManager::get().dirs())
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
            sources
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
            sources
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
        sources
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
