use crate::error::{AppError, AppResult};
use keyring::Entry;

const SERVICE: &str = "wisplingua";

pub struct SecretStore;

impl SecretStore {
    pub fn new() -> Self { Self }

    fn entry(&self, key: &str) -> AppResult<Entry> {
        Entry::new(SERVICE, key).map_err(AppError::from)
    }

    pub fn set(&self, key: &str, value: &str) -> AppResult<()> {
        self.entry(key)?.set_password(value).map_err(AppError::from)
    }

    pub fn get(&self, key: &str) -> AppResult<Option<String>> {
        match self.entry(key)?.get_password() {
            Ok(v) => Ok(Some(v)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(AppError::from(e)),
        }
    }

    pub fn delete(&self, key: &str) -> AppResult<()> {
        match self.entry(key)?.delete_credential() {
            Ok(_) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(AppError::from(e)),
        }
    }
}

pub const OPENROUTER_KEY: &str = "openrouter_api_key";
