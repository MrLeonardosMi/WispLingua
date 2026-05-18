use crate::config::TranslationStyle;
use crate::error::AppResult;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use serde::Serialize;

pub struct TranslateRequest {
    pub text: String,
    pub source: Option<String>,
    pub target: String,
    pub style: TranslationStyle,
    pub context: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TranslationChunk {
    Delta(String),
    Final(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub context_length: Option<u32>,
    pub pricing_prompt: Option<f64>,
    pub pricing_completion: Option<f64>,
    pub description: Option<String>,
}

pub trait TranslationProvider: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn current_model(&self) -> String;

    fn translate<'a>(
        &'a self,
        request: TranslateRequest,
    ) -> BoxFuture<'a, AppResult<BoxStream<'static, AppResult<TranslationChunk>>>>;

    fn list_models<'a>(&'a self) -> BoxFuture<'a, AppResult<Vec<ModelInfo>>>;

    fn test<'a>(&'a self) -> BoxFuture<'a, AppResult<String>>;
}
