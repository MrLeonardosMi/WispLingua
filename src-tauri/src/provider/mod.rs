pub mod openrouter;
pub mod types;

pub use types::{ModelInfo, TranslateRequest, TranslationChunk, TranslationProvider};

use crate::config::ProviderConfig;
use crate::error::{AppError, AppResult};
use openrouter::OpenRouterProvider;
use std::sync::Arc;

pub fn build_provider(cfg: &ProviderConfig, api_key: Option<String>) -> AppResult<Arc<dyn TranslationProvider>> {
    match cfg.active.as_str() {
        "openrouter" => {
            let key = api_key.ok_or(AppError::NoApiKey)?;
            Ok(Arc::new(OpenRouterProvider::new(cfg.openrouter.clone(), key)))
        }
        other => Err(AppError::Provider(format!("unknown provider: {other}"))),
    }
}
