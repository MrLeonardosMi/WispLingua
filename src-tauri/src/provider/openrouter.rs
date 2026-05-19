use std::time::Duration;

use eventsource_stream::Eventsource;
use futures::future::BoxFuture;
use futures::stream::{BoxStream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::{OpenRouterConfig, TranslationStyle};
use crate::error::{AppError, AppResult};
use crate::lang::lookup_name;

use super::types::{ModelInfo, TranslateRequest, TranslationChunk, TranslationProvider};

pub struct OpenRouterProvider {
    cfg: OpenRouterConfig,
    api_key: String,
    client: Client,
}

impl OpenRouterProvider {
    pub fn new(cfg: OpenRouterConfig, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(15))
            .build()
            .expect("reqwest client");
        Self { cfg, api_key, client }
    }

    fn build_messages(&self, req: &TranslateRequest) -> Vec<ChatMessage> {
        let style_hint = match req.style {
            TranslationStyle::Native => "Phrasing should sound natural to a native speaker, but never change the meaning, register, or sentence type of the source.",
            TranslationStyle::Formal => "Use a formal, professional register without changing meaning or sentence type.",
            TranslationStyle::Casual => "Use a casual, conversational register without changing meaning or sentence type.",
            TranslationStyle::Technical => "Preserve technical terminology, structure, and tone exactly.",
            TranslationStyle::Literal => "Translate as literally as faithful grammar allows, word-by-word where possible.",
        };
        let source_hint = match req.source.as_deref() {
            Some("auto") | None => "Detect the source language automatically.".to_string(),
            Some(code) => format!("The source language is {} ({}).", lookup_name(code), code),
        };
        let target_name = lookup_name(&req.target);
        let system = format!(
            "You are a professional translator. {source_hint} Translate the user's next message into {target_name} ({code}).\n\
\n\
HARD RULES (these override style, fluency, and intuition):\n\
1. Output ONLY the translation. No preamble, no commentary, no labels, no surrounding quotes, no notes, no alternatives.\n\
2. Preserve the SENTENCE TYPE exactly. If the source is a statement, the translation must be a statement. If it is a question, it must be a question. If it is an imperative, fragment, or exclamation, keep it that way. Never convert between these. Never add interrogative phrasing (\"Can I...\", \"May I...\", \"Should we...\", etc.) unless the source itself was a question.\n\
3. Preserve PUNCTUATION exactly. Do not add, remove, or change question marks, periods, exclamation marks, ellipses, dashes, brackets, or quotes. If the source ends without punctuation, the translation ends without punctuation. If the source ends with a period, end with a period. The presence and absence of every punctuation mark is meaningful.\n\
4. Preserve CASE and FORMATTING. Keep the original capitalization style (Title Case, lowercase, ALL CAPS, sentence case). Keep all line breaks, indentation, leading and trailing whitespace, markdown syntax, code blocks, URLs, numbers, emails, file paths, and proper nouns untouched. Translate prose around them only.\n\
5. Do not reinterpret ambiguity. If the source is ambiguous, render the most direct equivalent. Never add words to clarify intent that is not explicit in the source.\n\
6. {style_hint}\n\
\n\
Translate now.",
            code = req.target,
        );
        let mut messages = vec![ChatMessage { role: "system".into(), content: system }];
        if let Some(ctx) = &req.context {
            messages.push(ChatMessage {
                role: "system".into(),
                content: format!("Surrounding context for disambiguation only. Do not translate or output this context: {ctx}"),
            });
        }
        messages.push(ChatMessage { role: "user".into(), content: req.text.clone() });
        messages
    }

    fn auth_headers(&self) -> reqwest::header::HeaderMap {
        use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        headers.insert("HTTP-Referer", HeaderValue::from_str(&self.cfg.referrer).unwrap_or_else(|_| HeaderValue::from_static("")));
        headers.insert("X-Title", HeaderValue::from_str(&self.cfg.app_name).unwrap_or_else(|_| HeaderValue::from_static("WispLingua")));
        headers
    }
}

#[derive(Serialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct ChatChunkResponse {
    choices: Vec<ChatChunkChoice>,
}

#[derive(Deserialize, Debug)]
struct ChatChunkChoice {
    delta: ChatChunkDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct ChatChunkDelta {
    #[serde(default)]
    content: Option<String>,
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ApiModel>,
}

#[derive(Deserialize)]
struct ApiModel {
    id: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    context_length: Option<u32>,
    #[serde(default)]
    pricing: Option<ApiPricing>,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Deserialize)]
struct ApiPricing {
    #[serde(default)]
    prompt: Option<Value>,
    #[serde(default)]
    completion: Option<Value>,
}

fn parse_price(v: Option<Value>) -> Option<f64> {
    match v? {
        Value::String(s) => s.parse::<f64>().ok(),
        Value::Number(n) => n.as_f64(),
        _ => None,
    }
}

impl TranslationProvider for OpenRouterProvider {
    fn id(&self) -> &'static str { "openrouter" }
    fn display_name(&self) -> &'static str { "OpenRouter" }
    fn current_model(&self) -> String { self.cfg.model.clone() }

    fn translate<'a>(
        &'a self,
        request: TranslateRequest,
    ) -> BoxFuture<'a, AppResult<BoxStream<'static, AppResult<TranslationChunk>>>> {
        Box::pin(async move {
            let body = ChatRequest {
                model: self.cfg.model.clone(),
                messages: self.build_messages(&request),
                stream: true,
                temperature: self.cfg.temperature,
                max_tokens: self.cfg.max_tokens,
            };
            let url = format!("{}/chat/completions", self.cfg.base_url.trim_end_matches('/'));
            let resp = self
                .client
                .post(&url)
                .headers(self.auth_headers())
                .json(&body)
                .send()
                .await?;

            if !resp.status().is_success() {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                return Err(AppError::Provider(format!("openrouter {status}: {text}")));
            }

            let event_stream = resp.bytes_stream().eventsource();
            let stream = event_stream.filter_map(|res| async move {
                match res {
                    Ok(event) => {
                        let data = event.data;
                        if data.trim() == "[DONE]" {
                            return None;
                        }
                        match serde_json::from_str::<ChatChunkResponse>(&data) {
                            Ok(chunk) => {
                                let mut delta = String::new();
                                for c in chunk.choices {
                                    if let Some(t) = c.delta.content {
                                        delta.push_str(&t);
                                    }
                                }
                                if delta.is_empty() { None } else { Some(Ok(TranslationChunk::Delta(delta))) }
                            }
                            Err(_) => None,
                        }
                    }
                    Err(e) => Some(Err(AppError::Provider(format!("sse: {e}")))),
                }
            });

            Ok(Box::pin(stream) as BoxStream<'static, AppResult<TranslationChunk>>)
        })
    }

    fn list_models<'a>(&'a self) -> BoxFuture<'a, AppResult<Vec<ModelInfo>>> {
        Box::pin(async move {
            let url = format!("{}/models", self.cfg.base_url.trim_end_matches('/'));
            let resp = self.client.get(&url).headers(self.auth_headers()).send().await?;
            if !resp.status().is_success() {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                return Err(AppError::Provider(format!("openrouter models {status}: {text}")));
            }
            let body: ModelsResponse = resp.json().await?;
            let mut out: Vec<ModelInfo> = body
                .data
                .into_iter()
                .map(|m| ModelInfo {
                    name: m.name.clone().unwrap_or_else(|| m.id.clone()),
                    context_length: m.context_length,
                    pricing_prompt: parse_price(m.pricing.as_ref().and_then(|p| p.prompt.clone())),
                    pricing_completion: parse_price(m.pricing.as_ref().and_then(|p| p.completion.clone())),
                    description: m.description,
                    id: m.id,
                })
                .collect();
            out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            Ok(out)
        })
    }

    fn test<'a>(&'a self) -> BoxFuture<'a, AppResult<String>> {
        Box::pin(async move {
            let req = TranslateRequest {
                text: "Hello, world!".into(),
                source: Some("en".into()),
                target: "ru".into(),
                style: TranslationStyle::Native,
                context: None,
            };
            let mut stream = self.translate(req).await?;
            let mut acc = String::new();
            while let Some(chunk) = stream.next().await {
                if let Ok(TranslationChunk::Delta(s)) = chunk {
                    acc.push_str(&s);
                }
            }
            Ok(acc.trim().to_string())
        })
    }
}

