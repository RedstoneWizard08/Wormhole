#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(test)]
pub mod export;

use anyhow::Result;

use api::{plugin::PluginInfo, register::PLUGINS};

use data::{
    diesel::{
        r2d2::{ConnectionManager, Pool},
        ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
    },
    instance::{Instance, InstanceMeta},
    schema::{
        instance_meta::dsl::{game_id as gid, instance_id as iid, instance_meta},
        instances::dsl::{id, instances},
    },
};

#[cfg(test)]
use specta::{functions::FunctionDataType, ExportError, TypeDefs};
use tauri::State;

pub type DbState<'a> = State<'a, Pool<ConnectionManager<SqliteConnection>>>;

#[tauri::command]
#[specta::specta]
async fn launch(_instance_id: i32) {
    todo!()
}

#[tauri::command]
#[specta::specta]
fn get_instances(game_id: i32, pool: DbState<'_>) -> Result<Vec<Instance>> {
    let mut db = pool.get()?;

    let meta = instance_meta
        .filter(gid.eq(game_id))
        .select(InstanceMeta::as_select())
        .load(&mut db)?;

    let mut items = Vec::new();

    for item in meta {
        items.push(
            instances
                .filter(id.eq(item.instance_id))
                .select(Instance::as_select())
                .get_result(&mut db)?,
        );
    }

    Ok(items)
}

#[tauri::command]
#[specta::specta]
async fn get_instance(instance_id: i32, pool: DbState<'_>) -> Result<Instance> {
    Ok(instances
        .filter(id.eq(instance_id))
        .select(Instance::as_select())
        .get_result(&mut pool.get()?)?)
}

#[tauri::command]
#[specta::specta]
async fn get_instance_meta(instance_id: i32, pool: DbState<'_>) -> Result<InstanceMeta> {
    Ok(instance_meta
        .filter(iid.eq(instance_id))
        .select(InstanceMeta::as_select())
        .get_result(&mut pool.get()?)?)
}

#[allow(clippy::needless_range_loop)]
async fn levenshtein_distance(a: &str, b: &str) -> usize {
    let m = a.chars().count();
    let n = b.chars().count();
    let mut matrix = vec![vec![0; n + 1]; m + 1];

    if m == 0 {
        return n;
    }

    if n == 0 {
        return m;
    }

    for i in 0..=m {
        matrix[i][0] = i;
    }

    for j in 0..=n {
        matrix[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                0
            } else {
                1
            };

            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[m][n]
}

#[tauri::command]
#[specta::specta]
async fn get_distance(mod_name: &str, query: &str) -> Result<usize, String> {
    Ok(levenshtein_distance(query, mod_name).await)
}

#[tauri::command]
#[specta::specta]
async fn get_plugins() -> Vec<PluginInfo> {
    let mut res = Vec::new();

    for plugin in PLUGINS.lock().unwrap().values() {
        // For some reason, this doesn't implement Send or Sync,
        // even though the trait requires it and it's a bound on
        // the HashMap. So we have to use futures' block_on
        // function, which adds more dependencies to compile.
        // WTF, RUST

        res.push(futures::executor::block_on(plugin.as_info()));
    }

    res
}

#[macro_export]
macro_rules! funcs {
    ($ns: ident::$fn: ident) => {
        $ns::$fn![
            launch,
            get_instances,
            get_instance,
            get_distance,
            get_instance_meta,
            get_plugins,
        ]
    };
}

#[cfg(test)]
pub(crate) fn get_funcs() -> Result<(Vec<FunctionDataType>, TypeDefs), ExportError> {
    funcs!(specta::collect_types)
}

pub fn main() -> Result<()> {
    let db = init::boot()?;

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(funcs!(tauri::generate_handler))
        .run(tauri::generate_context!())
        .expect("Error while starting Wormhole!");

    Ok(())
}
