use once_cell::sync::Lazy;
use shuttle_runtime::SecretStore;
use tokio::sync::Mutex;
use std::error::Error;
use std::fmt;

pub const COLLECTION: &'static str = "i";
pub const MR_NOWMAN_MESSAGE: &'static str = "mnmm";
pub const MR_NOWMAN_CHAT: &'static str = "mnmc";

pub const A: &'static str = "a";
pub const REAL: &'static str = "r";
pub const I_ID: &'static str = "b4ea369a-d21e-40b4-afe7-4e84a4a7cd91";

// Bible verse processing constants
pub const BATCH_SIZE_BIBLE: usize = 10;
pub const QDRANT_TIMEOUT_SECS: u64 = 30;

pub static SECRETS: Lazy<Mutex<SecretStore>> =
    Lazy::new(|| Mutex::new(SecretStore::new(std::collections::BTreeMap::new())));

#[derive(Debug)]
pub struct SecretError(String);

impl fmt::Display for SecretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret error: {}", self.0)
    }
}

impl Error for SecretError {}

pub async fn get_secrets() -> Result<SecretStore, SecretError> {
    let secrets = SECRETS.lock().await;
    Ok(secrets.clone())
}