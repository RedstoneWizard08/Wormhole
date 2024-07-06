//! The router module.

use std::sync::Arc;

use data::prisma::PrismaClient;
use rpc_rs::{module::module::Module, proc::wrap, router::router::Router};

/// Create a router.
pub fn build_router() -> Router<Arc<PrismaClient>> {
    Router::<Arc<PrismaClient>>::new()
        .mount(
            "version",
            Module::builder()
                .read(wrap(|_cx, _: ()| async move { env!("CARGO_PKG_VERSION") }))
                .build(),
        )
        .mount(
            "mods",
            Module::builder()
                .read(wrap(|cx: Arc<PrismaClient>, _: ()| async move {
                    cx.r#mod()
                        .find_many(vec![])
                        .exec()
                        .await
                        .unwrap_or_default()
                }))
                .build(),
        )
}
