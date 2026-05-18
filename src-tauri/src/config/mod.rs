use std::fs;
use std::path::PathBuf;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: HotkeyConfig,
    pub target_language: String,
    pub fallback_target_language: String,
    pub style: TranslationStyle,
    pub provider: ProviderConfig,
    pub theme: Theme,
    pub autostart: bool,
    pub popup: PopupConfig,
    pub preserve_clipboard: bool,
    pub detect_source: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub trigger: HotkeyTrigger,
    pub chord_window_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HotkeyTrigger {
    Combo { combo: String, presses: u8 },
    Modifier { modifier: String, presses: u8 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TranslationStyle {
    Native,
    Formal,
    Casual,
    Technical,
    Literal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub active: String,
    pub openrouter: OpenRouterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterConfig {
    pub model: String,
    pub base_url: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub referrer: String,
    pub app_name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupConfig {
    pub width: u32,
    pub max_height: u32,
    pub follow_cursor: bool,
    pub prefer_below: bool,
    pub font_scale: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig {
                trigger: HotkeyTrigger::Combo {
                    combo: "Ctrl+C".to_string(),
                    presses: 2,
                },
                chord_window_ms: 350,
            },
            target_language: "en".to_string(),
            fallback_target_language: "ru".to_string(),
            style: TranslationStyle::Native,
            provider: ProviderConfig {
                active: "openrouter".to_string(),
                openrouter: OpenRouterConfig {
                    model: "openai/gpt-4o-mini".to_string(),
                    base_url: "https://openrouter.ai/api/v1".to_string(),
                    temperature: 0.2,
                    max_tokens: None,
                    referrer: "https://github.com/wisplingua/wisplingua".to_string(),
                    app_name: "WispLingua".to_string(),
                },
            },
            theme: Theme::Dark,
            autostart: false,
            popup: PopupConfig {
                width: 460,
                max_height: 320,
                follow_cursor: true,
                prefer_below: true,
                font_scale: 1.0,
            },
            preserve_clipboard: true,
            detect_source: true,
        }
    }
}

pub struct ConfigStore {
    path: PathBuf,
    inner: RwLock<AppConfig>,
}

impl ConfigStore {
    pub fn new(app_data_dir: PathBuf) -> AppResult<Self> {
        fs::create_dir_all(&app_data_dir).map_err(AppError::Io)?;
        let path = app_data_dir.join("config.json");
        let initial = if path.exists() {
            match fs::read_to_string(&path) {
                Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
                Err(_) => AppConfig::default(),
            }
        } else {
            let cfg = AppConfig::default();
            let _ = fs::write(&path, serde_json::to_string_pretty(&cfg)?);
            cfg
        };
        Ok(Self {
            path,
            inner: RwLock::new(initial),
        })
    }

    pub fn get(&self) -> AppConfig {
        self.inner.read().clone()
    }

    pub fn set(&self, cfg: AppConfig) -> AppResult<()> {
        let pretty = serde_json::to_string_pretty(&cfg)?;
        fs::write(&self.path, pretty).map_err(AppError::Io)?;
        *self.inner.write() = cfg;
        Ok(())
    }
}
