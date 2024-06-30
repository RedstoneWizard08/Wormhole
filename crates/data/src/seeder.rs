use std::sync::Arc;

use anyhow::Result;

use crate::{
    prisma::{source, PrismaClient},
    sources::Sources,
    sources_enum, Source,
};

pub async fn seed(client: Arc<PrismaClient>) -> Result<()> {
    client.source().create_many(
        Sources::values()
            .iter()
            .map(|it| {
                let src = it.create_source();

                source::create_unchecked(src.name.clone(), vec![source::name::set(src.name)])
            })
            .collect(),
    ).exec().await?;

    Ok(())
}
