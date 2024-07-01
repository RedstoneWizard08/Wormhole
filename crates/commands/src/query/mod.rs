//! Queries

use std::sync::Arc;

use data::{
    prisma::{r#mod, PrismaClient},
    Mod,
};
use rspc::{internal::MiddlewareBuilderLike, Error, ResultMarker, RouterBuilder};

async fn get_mods(db: Arc<PrismaClient>, instance: i32) -> Result<Vec<Mod>, Error> {
    db.r#mod()
        .find_many(vec![r#mod::instance_id::equals(instance)])
        .exec()
        .await
        .map_err(Into::into)
}

#[allow(missing_docs)]
pub fn mods<
    TMeta: Send + 'static,
    TMw: MiddlewareBuilderLike<Arc<PrismaClient>, LayerContext = Arc<PrismaClient>> + Send + 'static,
>(
    router: RouterBuilder<Arc<PrismaClient>, TMeta, TMw>,
) -> RouterBuilder<Arc<PrismaClient>, TMeta, TMw> {
    router.query_async::<ResultMarker, _, _, _>("mods", |t| {
        t(|db, instance: i32| get_mods(db, instance))
    })
}
