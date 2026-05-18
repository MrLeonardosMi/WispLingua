use std::time::Duration;

use arboard::Clipboard;
use enigo::{Direction, Enigo, Key as EnigoKey, Keyboard, Settings};
use tokio::time::sleep;

use crate::error::{AppError, AppResult};

pub struct Capture;

pub async fn replace_with(text: String, preserve: bool) -> AppResult<()> {
    let backup = if preserve { read_clipboard_safe().ok() } else { None };

    tokio::task::spawn_blocking({
        let text = text.clone();
        move || -> Result<(), String> {
            let mut cb = Clipboard::new().map_err(|e| e.to_string())?;
            cb.set_text(text).map_err(|e| e.to_string())
        }
    })
    .await
    .map_err(|e| AppError::Other(format!("join: {e}")))?
    .map_err(AppError::Other)?;

    sleep(Duration::from_millis(40)).await;

    tokio::task::spawn_blocking(send_paste)
        .await
        .map_err(|e| AppError::Other(format!("join: {e}")))?
        .map_err(AppError::Other)?;

    if let Some(prev) = backup {
        sleep(Duration::from_millis(180)).await;
        let _ = tokio::task::spawn_blocking(move || -> Result<(), String> {
            let mut cb = Clipboard::new().map_err(|e| e.to_string())?;
            cb.set_text(prev).map_err(|e| e.to_string())
        })
        .await;
    }

    Ok(())
}

fn send_paste() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    let modifier = if cfg!(target_os = "macos") {
        EnigoKey::Meta
    } else {
        EnigoKey::Control
    };
    enigo.key(modifier, Direction::Press).map_err(|e| e.to_string())?;
    std::thread::sleep(Duration::from_millis(15));
    enigo
        .key(EnigoKey::Unicode('v'), Direction::Click)
        .map_err(|e| e.to_string())?;
    std::thread::sleep(Duration::from_millis(10));
    enigo.key(modifier, Direction::Release).map_err(|e| e.to_string())?;
    Ok(())
}

impl Capture {
    pub async fn from_selection(simulate_copy: bool, preserve: bool) -> AppResult<String> {
        let backup = if preserve {
            read_clipboard_safe().ok()
        } else {
            None
        };

        if simulate_copy {
            tokio::task::spawn_blocking(send_copy)
                .await
                .map_err(|e| AppError::Other(format!("join: {e}")))?
                .map_err(|e| AppError::Other(format!("simulate copy: {e}")))?;
        }
        sleep(Duration::from_millis(if simulate_copy { 130 } else { 30 })).await;

        let text = read_clipboard_safe()
            .unwrap_or_default()
            .trim()
            .to_string();

        if let Some(prev) = backup {
            sleep(Duration::from_millis(80)).await;
            let _ = write_clipboard_safe(&prev);
        }

        if text.is_empty() {
            return Err(AppError::NoInput);
        }
        Ok(text)
    }

    pub fn read_clipboard() -> AppResult<String> {
        read_clipboard_safe()
    }
}

fn read_clipboard_safe() -> AppResult<String> {
    let mut cb = Clipboard::new()?;
    Ok(cb.get_text()?)
}

fn write_clipboard_safe(text: &str) -> AppResult<()> {
    let mut cb = Clipboard::new()?;
    cb.set_text(text.to_string())?;
    Ok(())
}

fn send_copy() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    let modifier = if cfg!(target_os = "macos") {
        EnigoKey::Meta
    } else {
        EnigoKey::Control
    };
    enigo.key(modifier, Direction::Press).map_err(|e| e.to_string())?;
    std::thread::sleep(Duration::from_millis(15));
    enigo
        .key(EnigoKey::Unicode('c'), Direction::Click)
        .map_err(|e| e.to_string())?;
    std::thread::sleep(Duration::from_millis(10));
    enigo.key(modifier, Direction::Release).map_err(|e| e.to_string())?;
    Ok(())
}
