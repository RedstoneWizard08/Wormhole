//! The router module.

use std::sync::Arc;

use data::prisma::{game, instance, r#mod, source, PrismaClient};
use rpc_rs::{module::module::Module, prisma_module, proc::wrap, router::router::Router};

/// Create a router.
pub fn build_router() -> Router<Arc<PrismaClient>> {
    let mut router = Router::<Arc<PrismaClient>>::new().mount(
        "version",
        Module::builder()
            .read(wrap(|_cx, _: ()| async move { env!("CARGO_PKG_VERSION") }))
            .build(),
    );

    prisma_module!(router += ["mod", "mods"] {
        client: PrismaClient,
        module: r#mod,
        container: r#mod,
        primary_key: id,
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

    prisma_module!(router += ["instance", "instances"] {
        client: PrismaClient,
        module: instance,
        container: instance,
        primary_key: id,
    });

    router
}
