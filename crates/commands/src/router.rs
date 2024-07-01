//! The router module.

use std::sync::Arc;

use data::prisma::{r#mod, PrismaClient};
use rspc::Router;

/// Create a router.
pub fn build_router() -> Router<Arc<PrismaClient>> {
    Router::<Arc<PrismaClient>>::new()
        .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
        .query("mods", |t| {
            t(|db, instance: i32| async move {
                db.r#mod()
                    .find_many(vec![r#mod::instance_id::equals(instance)])
                    .exec()
                    .await
                    .unwrap_or_default()
            })
        })
        .build()
}
