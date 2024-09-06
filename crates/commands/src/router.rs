use std::sync::Arc;

use data::prisma::{installed_mod, instance, PrismaClient};
use rpc_rs::{prisma_module_filtered, proc::wrap, Router};
use whcore::CoreManager;

pub fn build_router() -> Router<Arc<PrismaClient>> {
    let mut router = Router::new()
        .invoke(
            "version",
            wrap(|_, _: ()| async move { env!("CARGO_PKG_VERSION") }),
        )
        .invoke(
            "dataDir",
            wrap(|_, _: ()| async move { CoreManager::get().root() }),
        )
        .invoke(
            "gameDir",
            wrap(|_, game: String| async move { CoreManager::get().game_data_dir(game) }),
        );

    prisma_module_filtered!(router += ["instance", "instances"] {
        client: PrismaClient,
        module: instance,
        container: instance,
        primary_key: id,
        filter: game_id = i32,
    });

    prisma_module_filtered!(router += ["mod", "mods"] {
        client: PrismaClient,
        module: installed_mod,
        container: installed_mod,
        primary_key: id,
        filter: instance_id = i32,
    });

    router
}
