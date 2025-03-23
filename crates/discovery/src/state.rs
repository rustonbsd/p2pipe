use std::sync::Arc;

use mainline::Dht;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

pub static DHT: OnceCell<Arc<Mutex<Dht>>> = OnceCell::new();

pub fn get_dht() -> Arc<Mutex<Dht>> {
    DHT.get_or_init(||{
        let dht = Dht::server().expect("failed to initialize dht");
        Arc::new(Mutex::new(dht))
    }).clone()
}