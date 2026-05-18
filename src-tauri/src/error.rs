use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("config error: {0}")]
    Config(String),

    #[error("provider error: {0}")]
    Provider(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("clipboard error: {0}")]
    Clipboard(String),

    #[error("keyring error: {0}")]
    Keyring(String),

    #[error("tauri error: {0}")]
    Tauri(String),

    #[error("hotkey error: {0}")]
    Hotkey(String),

    #[error("no api key configured")]
    NoApiKey,

    #[error("no input text")]
    NoInput,

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(e: tauri::Error) -> Self { AppError::Tauri(e.to_string()) }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self { AppError::Network(e.to_string()) }
}

impl From<keyring::Error> for AppError {
    fn from(e: keyring::Error) -> Self { AppError::Keyring(e.to_string()) }
}

impl From<arboard::Error> for AppError {
    fn from(e: arboard::Error) -> Self { AppError::Clipboard(e.to_string()) }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self { AppError::Other(e.to_string()) }
}

pub type AppResult<T> = Result<T, AppError>;
