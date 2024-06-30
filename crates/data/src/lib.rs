#![allow(unused)]

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use prisma::PrismaClient;
use prisma_client_rust::specta::{NamedType, TypeMap};
use tokio::sync::OnceCell;

pub mod macros;
pub mod prisma;
pub mod seeder;
pub mod sources;

pub type Game = prisma::game::Data;
pub type Source = prisma::source::Data;
pub type Instance = prisma::instance::Data;
pub type Mod = prisma::r#mod::Data;

pub const CLIENT: OnceCell<Arc<PrismaClient>> = OnceCell::const_new();

pub async fn client() -> Result<PrismaClient> {
    Ok(PrismaClient::_builder().build().await?)
}

async fn arc_client() -> Result<Arc<PrismaClient>> {
    client().await.map(Arc::new)
}

pub async fn get_or_init_client() -> Result<Arc<PrismaClient>> {
    CLIENT.get_or_try_init(arc_client).await.cloned()
}

pub fn type_map() -> TypeMap {
    let mut map = TypeMap::default();

    let ty = Game::named_data_type(&mut map, &[]);
    map.insert(Game::sid(), ty);

    let ty = Source::named_data_type(&mut map, &[]);
    map.insert(Source::sid(), ty);

    let ty = Instance::named_data_type(&mut map, &[]);
    map.insert(Instance::sid(), ty);

    let ty = Mod::named_data_type(&mut map, &[]);
    map.insert(Mod::sid(), ty);

    map
}

impl Instance {
    pub fn data_dir(&self) -> PathBuf {
        PathBuf::from(&self.data_dir)
    }

    pub fn cache_dir(&self) -> PathBuf {
        PathBuf::from(&self.cache_dir)
    }

    pub fn install_dir(&self) -> PathBuf {
        PathBuf::from(&self.install_dir)
    }
}
