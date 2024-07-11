//! The router module.

use crate::{
    base::get_plugins,
    loader::{get_latest_loader, get_loaders},
};
use data::prisma::{game, instance, r#mod, source, PrismaClient};
use mcmeta::cmd::modded::ModLoaderType;
use rpc_rs::{
    module::module::Module, prisma_module, prisma_module_filtered, proc::wrap,
    router::router::Router,
};
use std::sync::Arc;
use whcore::{errors::Stringify, manager::CoreManager};

/// Create a router.
pub fn build_router() -> Router<Arc<PrismaClient>> {
    let mut router = Router::<Arc<PrismaClient>>::new().invoke(
        "version",
        wrap(|_cx, _: ()| async move { env!("CARGO_PKG_VERSION") }),
    );

    prisma_module_filtered!(router += ["mod", "mods"] {
        client: PrismaClient,
        module: r#mod,
        container: r#mod,
        primary_key: id,
        filter: instance_id = i32,
    });

    prisma_module!(router += ["game", "games"] {
        client: PrismaClient,
        module: game,
        container: game,
        primary_key: id,
    });

    prisma_module!(router += ["source", "sources"] {
        client: PrismaClient,
        module: source,
        container: source,
        primary_key: id,
    });

    prisma_module_filtered!(router += ["instance", "instances"] {
        client: PrismaClient,
        module: instance,
        container: instance,
        primary_key: id,
        filter: game_id = i32,
    });

    router
        .mount(
            "plugins",
            Module::builder()
                .read(wrap(
                    |_cx, _: ()| async move { get_plugins().await.stringify() },
                ))
                .build(),
        )
        .mount(
            "latestLoader",
            Module::builder()
                .read(wrap(|_cx, ty: ModLoaderType| async move {
                    get_latest_loader(ty).await.stringify()
                }))
                .build(),
        )
        .mount(
            "loaders",
            Module::builder()
                .read(wrap(|_cx, ty: ModLoaderType| async move {
                    get_loaders(ty).await.stringify()
                }))
                .build(),
        )
        .mount(
            "dirs",
            Module::builder()
                .read(wrap(|_cx, game: String| async move {
                    CoreManager::get().game_dirs(game)
                }))
                .build(),
        )
}
