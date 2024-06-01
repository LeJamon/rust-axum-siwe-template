use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/*
This code is not suitable for production use.
It is recommended to use a database to store and get the nonce in production.
*/
lazy_static! {
    pub static ref NONCE_CACHE: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));
}
pub async fn set_nonce(key: String, value: String) {
    let mut cache = NONCE_CACHE.lock().await;
    cache.insert(key, value);
}

pub async fn get_nonce(key: &str) -> Option<String> {
    let mut cache = NONCE_CACHE.lock().await;
    cache.remove(key)
}
