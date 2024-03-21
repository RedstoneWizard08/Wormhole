use std::{env, sync::Mutex};

use ckandex::{refresh_data, CacheClient, KSP};
use lazy_static::lazy_static;

use crate::util::get_data_dir;

pub mod query;

lazy_static! {
    pub static ref KSP1_CACHE_CLIENT: Mutex<Option<CacheClient>> = Mutex::new(None);
    pub static ref KSP2_CACHE_CLIENT: Mutex<Option<CacheClient>> = Mutex::new(None);
    pub static ref GLOBAL_TOKEN: Mutex<String> = Mutex::new(String::new());
}

pub async fn setup_ckan() {
    let ckan_data_dir = get_data_dir().join("CKAN");

    let ksp1_data_dir = ckan_data_dir.join("KSP1");
    let ksp2_data_dir = ckan_data_dir.join("KSP2");

    let ksp1_data_dir_str = ksp1_data_dir.as_os_str().to_str().unwrap();
    let ksp2_data_dir_str = ksp2_data_dir.as_os_str().to_str().unwrap();

    refresh_data(KSP::KSP1, ksp1_data_dir_str).await;
    refresh_data(KSP::KSP2, ksp2_data_dir_str).await;

    *KSP1_CACHE_CLIENT.lock().unwrap() = Some(CacheClient::new(ksp1_data_dir_str.to_string()));
    *KSP2_CACHE_CLIENT.lock().unwrap() = Some(CacheClient::new(ksp2_data_dir_str.to_string()));

    let mut cache_ksp1 = KSP1_CACHE_CLIENT.lock().unwrap().clone().unwrap();
    let mut cache_ksp2 = KSP2_CACHE_CLIENT.lock().unwrap().clone().unwrap();

    cache_ksp1.update_cache().await.unwrap();
    cache_ksp2.update_cache().await.unwrap();

    *KSP1_CACHE_CLIENT.lock().unwrap() = Some(cache_ksp1);
    *KSP2_CACHE_CLIENT.lock().unwrap() = Some(cache_ksp2);

    let token = env::var("GITHUB_TOKEN");

    if let Ok(token) = token {
        *GLOBAL_TOKEN.lock().unwrap() = token;
    }
}
