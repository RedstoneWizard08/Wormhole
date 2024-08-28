use std::sync::Arc;

use anyhow::Result;

use crate::prisma::PrismaClient;

pub async fn seed(_client: Arc<PrismaClient>) -> Result<()> {
    Ok(())
}
