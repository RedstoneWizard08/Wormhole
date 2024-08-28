#![allow(unused)]

pub mod prisma;
pub mod seeder;

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use prisma::PrismaClient;
use prisma_client_rust::specta::{NamedType, TypeMap};
use tokio::sync::OnceCell;
use whcore::{async_trait::async_trait, type_map};

#[async_trait]
pub trait DbIntoAsync<T> {
    async fn db_into_async(self, client: Arc<PrismaClient>) -> T;
}

pub const CLIENT: OnceCell<Arc<PrismaClient>> = OnceCell::const_new();

async fn client() -> Result<Arc<PrismaClient>> {
    let client = Arc::new(PrismaClient::_builder().build().await?);

    seeder::seed(client.clone()).await?;

    Ok(client)
}

pub fn get_client() -> Arc<PrismaClient> {
    CLIENT.get().unwrap().clone()
}

pub async fn get_or_init_client() -> Result<Arc<PrismaClient>> {
    CLIENT.get_or_try_init(client).await.cloned()
}

type_map! {}
