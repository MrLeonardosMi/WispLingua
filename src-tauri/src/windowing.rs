use tauri::{AppHandle, LogicalSize, Manager, PhysicalPosition, WebviewWindow};
use tokio::time::{sleep, Duration};

use crate::config::PopupConfig;
use crate::cursor::place_popup;
use crate::error::{AppError, AppResult};
use crate::hotkey::listener::CursorPos;

pub fn popup(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window("popup")
}

pub fn settings_win(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window("settings")
}

pub fn position_popup(app: &AppHandle, cursor: CursorPos, cfg: &PopupConfig) -> (i32, i32) {
    if !cfg.follow_cursor {
        if let Some(m) = app.primary_monitor().ok().flatten() {
            let p = m.position();
            let s = m.size();
            let x = p.x + (s.width as i32 - cfg.width as i32) / 2;
            let y = p.y + (s.height as i32 - cfg.max_height as i32) / 2;
            return (x, y);
        }
        return (200, 200);
    }
    let placement = place_popup(app, cursor, cfg.width, cfg.max_height, cfg.prefer_below);
    (placement.x, placement.y)
}

pub async fn ensure_popup_visible(
    app: &AppHandle,
    x: i32,
    y: i32,
    width: u32,
    max_height: u32,
) -> AppResult<()> {
    let win = popup(app).ok_or_else(|| AppError::Tauri("popup window missing".into()))?;
    win.set_size(LogicalSize::new(width as f64, max_height as f64))?;
    win.set_position(PhysicalPosition::new(x, y))?;
    if !win.is_visible().unwrap_or(false) {
        win.show()?;
    }
    win.set_focus()?;
    sleep(Duration::from_millis(20)).await;
    Ok(())
}

pub fn hide_popup(app: &AppHandle) -> AppResult<()> {
    if let Some(w) = popup(app) {
        w.hide()?;
    }
    Ok(())
}

pub fn pin_popup(app: &AppHandle, _pinned: bool) -> AppResult<()> {
    if let Some(w) = popup(app) {
        w.set_always_on_top(true)?;
    }
    Ok(())
}

pub fn show_settings(app: &AppHandle) -> AppResult<()> {
    if let Some(w) = settings_win(app) {
        if !w.is_visible().unwrap_or(false) {
            w.show()?;
        }
        w.unminimize().ok();
        w.set_focus()?;
    }
    Ok(())
}
