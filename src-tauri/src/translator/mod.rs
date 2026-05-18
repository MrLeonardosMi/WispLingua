use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use futures::StreamExt;
use parking_lot::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

use crate::capture::Capture;
use crate::config::AppConfig;
use crate::error::{AppError, AppResult};
use crate::hotkey::{is_copy_combo, parse_trigger};
use crate::lang::detect_language;
use crate::provider::{build_provider, TranslateRequest, TranslationChunk};
use crate::secret::{SecretStore, OPENROUTER_KEY};
use crate::state::AppState;
use crate::windowing::{ensure_popup_visible, position_popup};

#[derive(Clone, Serialize)]
struct StartPayload {
    request_id: String,
    source_text: String,
    source_language: Option<String>,
    target_language: String,
    model: String,
    cursor_x: i32,
    cursor_y: i32,
}

#[derive(Clone, Serialize)]
struct ChunkPayload {
    request_id: String,
    delta: String,
}

#[derive(Clone, Serialize)]
struct DonePayload {
    request_id: String,
    total: String,
    elapsed_ms: u128,
}

#[derive(Clone, Serialize)]
struct ErrorPayload {
    request_id: String,
    message: String,
}

pub struct Translator {
    pub current: Mutex<Option<TranslationHandle>>,
}

pub struct TranslationHandle {
    pub id: String,
    pub cancel: Arc<AtomicBool>,
}

impl Translator {
    pub fn new() -> Self {
        Self { current: Mutex::new(None) }
    }

    pub fn cancel_all(&self) {
        if let Some(h) = self.current.lock().take() {
            h.cancel.store(true, Ordering::Relaxed);
        }
    }

    pub fn cancel_by_id(&self, id: &str) {
        let mut guard = self.current.lock();
        if let Some(h) = guard.as_ref() {
            if h.id == id {
                h.cancel.store(true, Ordering::Relaxed);
                *guard = None;
            }
        }
    }
}

pub async fn run_from_selection(app: AppHandle) -> AppResult<()> {
    let state: tauri::State<AppState> = app.state();
    let cfg = state.config.get();
    let parsed = parse_trigger(&cfg.hotkey.trigger)?;
    let simulate = !is_copy_combo(&parsed);

    let prev = crate::desktop::current_foreground();
    *state.previous_window.lock() = prev;

    let text = Capture::from_selection(simulate, cfg.preserve_clipboard).await?;
    run_with_text(app, text).await
}

pub async fn run_with_text(app: AppHandle, text: String) -> AppResult<()> {
    let text = text.trim().to_string();
    if text.is_empty() {
        return Err(AppError::NoInput);
    }

    let state: tauri::State<AppState> = app.state();
    let cfg = state.config.get();

    let detected = if cfg.detect_source { detect_language(&text) } else { None };
    let target = pick_target(&cfg, detected.as_deref());

    let api_key = state.secrets.get(OPENROUTER_KEY)?;
    let provider = build_provider(&cfg.provider, api_key)?;
    let model = provider.current_model();

    state.translator.cancel_all();
    let request_id = Uuid::new_v4().to_string();
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut guard = state.translator.current.lock();
        *guard = Some(TranslationHandle { id: request_id.clone(), cancel: cancel.clone() });
    }

    let cursor = state.hotkey.cursor();
    let (px, py) = position_popup(&app, cursor, &cfg.popup);
    ensure_popup_visible(&app, px, py, cfg.popup.width, cfg.popup.max_height).await?;

    let _ = app.emit(
        "translation://start",
        StartPayload {
            request_id: request_id.clone(),
            source_text: text.clone(),
            source_language: detected.clone(),
            target_language: target.clone(),
            model: model.clone(),
            cursor_x: cursor.x,
            cursor_y: cursor.y,
        },
    );

    let started = Instant::now();
    let mut full = String::new();

    let stream_res = provider
        .translate(TranslateRequest {
            text: text.clone(),
            source: Some("auto".to_string()),
            target: target.clone(),
            style: cfg.style,
            context: None,
        })
        .await;

    let mut stream = match stream_res {
        Ok(s) => s,
        Err(e) => {
            let _ = app.emit(
                "translation://error",
                ErrorPayload { request_id: request_id.clone(), message: e.to_string() },
            );
            return Err(e);
        }
    };

    while let Some(chunk) = stream.next().await {
        if cancel.load(Ordering::Relaxed) {
            break;
        }
        match chunk {
            Ok(TranslationChunk::Delta(s)) => {
                full.push_str(&s);
                let _ = app.emit(
                    "translation://chunk",
                    ChunkPayload { request_id: request_id.clone(), delta: s },
                );
            }
            Ok(TranslationChunk::Final(s)) => {
                full = s;
            }
            Err(e) => {
                let _ = app.emit(
                    "translation://error",
                    ErrorPayload { request_id: request_id.clone(), message: e.to_string() },
                );
                return Err(e);
            }
        }
    }

    let elapsed = started.elapsed().as_millis();
    let _ = app.emit(
        "translation://done",
        DonePayload { request_id: request_id.clone(), total: full, elapsed_ms: elapsed },
    );

    let mut guard = state.translator.current.lock();
    if let Some(h) = guard.as_ref() {
        if h.id == request_id {
            *guard = None;
        }
    }
    Ok(())
}

fn pick_target(cfg: &AppConfig, detected: Option<&str>) -> String {
    let primary = cfg.target_language.clone();
    let secondary = cfg.fallback_target_language.clone();
    match detected {
        Some(src) if src == primary => secondary,
        Some(src) if src == secondary => primary,
        _ => primary,
    }
}
