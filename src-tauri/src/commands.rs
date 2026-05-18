use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

use crate::config::AppConfig;
use crate::error::{AppError, AppResult};
use crate::lang::{supported, LanguageOption};
use crate::provider::{build_provider, ModelInfo};
use crate::secret::OPENROUTER_KEY;
use crate::state::AppState;
use crate::translator::{run_from_selection, run_with_text};
use crate::windowing::{hide_popup, show_settings};

#[tauri::command]
pub fn get_config(state: tauri::State<'_, AppState>) -> AppConfig {
    state.config.get()
}

#[tauri::command]
pub fn save_config(state: tauri::State<'_, AppState>, config: AppConfig) -> AppResult<()> {
    state.config.set(config.clone())?;
    state.hotkey.apply(&config.hotkey)?;
    Ok(())
}

#[tauri::command]
pub fn list_languages() -> Vec<LanguageOption> {
    supported()
}

#[tauri::command]
pub fn get_api_key(state: tauri::State<'_, AppState>) -> AppResult<Option<String>> {
    let v = state.secrets.get(OPENROUTER_KEY)?;
    Ok(v.map(|_| "***".to_string()))
}

#[tauri::command]
pub fn set_api_key(state: tauri::State<'_, AppState>, key: String) -> AppResult<()> {
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return Err(AppError::Other("empty key".into()));
    }
    state.secrets.set(OPENROUTER_KEY, trimmed)
}

#[tauri::command]
pub fn clear_api_key(state: tauri::State<'_, AppState>) -> AppResult<()> {
    state.secrets.delete(OPENROUTER_KEY)
}

#[tauri::command]
pub async fn fetch_models(state: tauri::State<'_, AppState>) -> AppResult<Vec<ModelInfo>> {
    let cfg = state.config.get();
    let key = state.secrets.get(OPENROUTER_KEY)?;
    let provider = build_provider(&cfg.provider, key)?;
    provider.list_models().await
}

#[tauri::command]
pub async fn translate_text(app: AppHandle, text: String) -> AppResult<()> {
    run_with_text(app, text).await
}

#[tauri::command]
pub async fn translate_selection(app: AppHandle) -> AppResult<()> {
    run_from_selection(app).await
}

#[tauri::command]
pub fn cancel_translation(state: tauri::State<'_, AppState>, request_id: String) -> AppResult<()> {
    state.translator.cancel_by_id(&request_id);
    Ok(())
}

#[tauri::command]
pub fn close_popup(app: AppHandle, state: tauri::State<'_, AppState>) -> AppResult<()> {
    state.translator.cancel_all();
    hide_popup(&app)
}

#[tauri::command]
pub fn pin_popup(app: AppHandle, pinned: bool) -> AppResult<()> {
    crate::windowing::pin_popup(&app, pinned)
}

#[tauri::command]
pub fn open_settings(app: AppHandle) -> AppResult<()> {
    show_settings(&app)
}

#[tauri::command]
pub fn apply_hotkey(state: tauri::State<'_, AppState>) -> AppResult<()> {
    let cfg = state.config.get();
    state.hotkey.apply(&cfg.hotkey)
}

#[tauri::command]
pub fn apply_autostart(app: AppHandle, state: tauri::State<'_, AppState>) -> AppResult<()> {
    let want = state.config.get().autostart;
    let mgr = app.autolaunch();
    let enabled = mgr.is_enabled().unwrap_or(false);
    if want && !enabled {
        mgr.enable().map_err(|e| AppError::Other(e.to_string()))?;
    } else if !want && enabled {
        mgr.disable().map_err(|e| AppError::Other(e.to_string()))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn replace_selection(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    text: String,
) -> AppResult<()> {
    let text = text.trim_end().to_string();
    if text.is_empty() {
        return Err(AppError::Other("empty replacement".into()));
    }
    let preserve = state.config.get().preserve_clipboard;
    let prev = *state.previous_window.lock();

    crate::windowing::hide_popup(&app)?;
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    if let Some(hwnd) = prev {
        crate::desktop::focus(hwnd);
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    } else {
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    }

    crate::capture::replace_with(text, preserve).await
}

#[tauri::command]
pub fn swap_languages(state: tauri::State<'_, AppState>) -> AppResult<AppConfig> {
    let mut cfg = state.config.get();
    std::mem::swap(&mut cfg.target_language, &mut cfg.fallback_target_language);
    state.config.set(cfg.clone())?;
    Ok(cfg)
}

#[tauri::command]
pub async fn test_provider(state: tauri::State<'_, AppState>) -> AppResult<String> {
    let cfg = state.config.get();
    let key = state.secrets.get(OPENROUTER_KEY)?;
    let provider = build_provider(&cfg.provider, key)?;
    provider.test().await
}
