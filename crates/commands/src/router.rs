use std::sync::Arc;

use data::prisma::{PrismaClient, instance, installed_mod};
use rpc_rs::{prisma_module, prisma_module_filtered, Router};

pub fn build_router() -> Router<Arc<PrismaClient>> {
    let mut router = Router::new();

    prisma_module!(router += ["instance", "instances"] {
        client: PrismaClient,
        module: instance,
        container: instance,
        primary_key: id,
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
