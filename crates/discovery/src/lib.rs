mod utils;
mod state;

use anyhow::{bail, Result};
use ed25519_dalek::SecretKey;
use mainline::{Id, MutableItem, SigningKey};
use state::get_dht;
use utils::time_now;

pub async fn announce(secret_key: SecretKey,value: &[u8]) -> Result<Id> {

    let signer = SigningKey::try_from(secret_key.to_vec().as_slice())?;
    let seq: i64 = time_now() as i64;
    let mut_item = MutableItem::new(signer.clone(),value,seq,None);

    let dht = get_dht();
    let dht_locked = dht.lock().await;
    match dht_locked.put_mutable(mut_item.clone(),Some(seq)) {
        Ok(item_id) => Ok(item_id),
        Err(err) => bail!("announce: dht.put_mutable failed in mainline: {err}"),
    }
}

pub async fn()

