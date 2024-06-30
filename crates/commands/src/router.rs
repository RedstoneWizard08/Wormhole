//! The router module.

use std::sync::Arc;

use data::prisma::PrismaClient;
use midlog::midlog_log;
use rspc::Router;

/// Create a router.
pub fn build_router() -> Router<Arc<PrismaClient>> {
    Router::new()
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                midlog_log!(mw.req.kind.to_str(), mw.req.path.as_str(), 200, 0);
                Ok(mw)
            })
        })
        .query("version", |t| t(|_ctx, _input: ()| env!("CARGO_PKG_VERSION")))
        .build()
}
