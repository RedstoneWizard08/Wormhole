use ckandex::{kref::resolve_kref, CKANError, IdFilter, QueryBuilder, QueryResponse};

use crate::instances::KSPGame;

use super::{GLOBAL_TOKEN, KSP1_CACHE_CLIENT, KSP2_CACHE_CLIENT};

pub async fn get_mods(game: KSPGame) -> QueryResponse {
    let query = QueryBuilder::new();
    let query = query.build();

    let mut client = KSP1_CACHE_CLIENT.lock().unwrap();

    if game == KSPGame::KSP2 {
        client = KSP2_CACHE_CLIENT.lock().unwrap();
    }

    let client = client.clone().unwrap();
    let resp = query.execute(client);

    return resp;
}

pub async fn get_mod(game: KSPGame, mod_id: String) -> Result<String, CKANError> {
    let mut query = QueryBuilder::new();
    let query = query.add(IdFilter::new(mod_id)).build();

    let mut client = KSP1_CACHE_CLIENT.lock().unwrap();

    if game == KSPGame::KSP2 {
        client = KSP2_CACHE_CLIENT.lock().unwrap();
    }

    let client = client.clone().unwrap();
    let resp = query.execute(client);

    let item = resp.first();

    if let Some(item) = item {
        let resolved = resolve_kref(item.kref, GLOBAL_TOKEN.lock().unwrap().clone()).await;

        return resolved;
    }

    return Err(CKANError::UnknownMod);
}
