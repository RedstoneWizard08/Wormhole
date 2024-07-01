//! The router module.

use std::sync::Arc;

use data::prisma::PrismaClient;
use midlog::midlog_log;
use rspc::Router;

use crate::apply;

/// Create a router.
pub fn build_router() -> Router<Arc<PrismaClient>> {
    let router = Router::new()
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                midlog_log!(mw.req.kind.to_str(), mw.req.path.as_str(), 200, 0);
                Ok(mw)
            })
        })
        .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")));

    apply!(
        router:
            crate::query::mods
    );

    router.build()
}
