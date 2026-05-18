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
            TranslationStyle::Native => "Use idiomatic phrasing that sounds native.",
            TranslationStyle::Formal => "Use a formal, professional register.",
            TranslationStyle::Casual => "Use a casual, conversational register.",
            TranslationStyle::Technical => "Preserve technical terminology and structure precisely.",
            TranslationStyle::Literal => "Translate as literally as faithful grammar allows.",
        };
        let source_hint = match req.source.as_deref() {
            Some("auto") | None => "Detect the source language automatically.".to_string(),
            Some(code) => format!("The source language is {} ({}).", lookup_name(code), code),
        };
        let target_name = lookup_name(&req.target);
        let system = format!(
            "You are a professional translator. {source_hint} Translate the user's text into {target_name} ({}). {style_hint} \
Preserve formatting, punctuation, line breaks, markdown, code blocks, URLs, numbers, and proper nouns. \
Do NOT add any commentary, quotation marks around the result, or labels. Output ONLY the translation.",
            req.target
        );
        let mut messages = vec![ChatMessage { role: "system".into(), content: system }];
        if let Some(ctx) = &req.context {
            messages.push(ChatMessage {
                role: "system".into(),
                content: format!("Surrounding context for disambiguation only (do not translate this): {ctx}"),
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

