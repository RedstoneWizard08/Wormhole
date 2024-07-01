use std::sync::Arc;

use anyhow::Result;

use crate::{
    prisma::{source, PrismaClient},
    sources::Sources,
    sources_enum, Source,
};

pub async fn seed(client: Arc<PrismaClient>) -> Result<()> {
    for source in Sources::values() {
        let src = source.create_source();
        let it = source::create(src.name.clone(), vec![source::name::set(src.name.clone())]);

        client
            .source()
            .upsert(source::name::equals(src.name), it.clone(), it.to_params())
            .exec()
            .await?;
    }

    Ok(())
}
