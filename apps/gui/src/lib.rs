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
};

use specta::functions::CollectFunctionsResult;
use tauri::{utils::assets::EmbeddedAssets, Context, Invoke, Runtime};
use whcore::{merge_type_maps, state::TState, Boolify};

pub type AppState<'a> = TState<'a, Pool<ConnectionManager<SqliteConnection>>>;

#[macros::serde_call]
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

#[macros::serde_call]
#[tauri::command]
#[specta::specta]
async fn delete_instance(instance_id: i32, pool: AppState<'_>) -> Result<(), bool> {
    let mut db = pool.get().bool()?;

    delete(instances.filter(id.eq(instance_id)))
        .execute(&mut db)
        .bool()?;

    Ok(())
}

#[macros::serde_call]
#[tauri::command]
#[specta::specta]
async fn add_instance(instance: Instance, pool: AppState<'_>) -> Result<Instance, bool> {
    Ok(insert_into(instances)
        .values(instance)
        .returning(Instance::as_returning())
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}

#[macros::serde_call]
#[tauri::command]
#[specta::specta]
async fn get_instance(instance_id: i32, pool: AppState<'_>) -> Result<Instance, bool> {
    Ok(instances
        .filter(id.eq(instance_id))
        .select(Instance::as_select())
        .get_result(&mut pool.get().bool()?)
        .bool()?)
}

#[macros::serde_call]
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

#[macro_export]
macro_rules! funcs {
    ($ns: ident::$fn: ident) => {
        $ns::$fn![
            get_instances,
            get_instance,
            get_plugins,
            add_instance,
            delete_instance,
        ]
    };

    ($ns: ident::$fn: ident;) => {
        $ns::$fn![
            get_instances,
            get_instance,
            get_plugins,
            add_instance,
            delete_instance
        ];
    };
}

funcs!(macros::serde_funcs;);

pub fn funcs() -> CollectFunctionsResult {
    let map = merge_type_maps(vec![data::type_map(), api::type_map()]);

    specta::functions::collect_functions![map; get_instances, get_instance, get_plugins, add_instance, delete_instance]
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
