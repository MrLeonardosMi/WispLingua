pub mod capture;
pub mod commands;
pub mod config;
pub mod cursor;
pub mod desktop;
pub mod error;
pub mod hotkey;
pub mod lang;
pub mod provider;
pub mod secret;
pub mod state;
pub mod translator;
pub mod tray;
pub mod windowing;

use std::sync::Arc;

use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

use crate::config::ConfigStore;
use crate::hotkey::HotkeyService;
use crate::secret::SecretStore;
use crate::state::AppState;
use crate::translator::Translator;

pub fn run() {
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,wisplingua_lib=debug"),
    )
    .try_init();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_config_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));
            let config = Arc::new(ConfigStore::new(app_data_dir)?);
            let secrets = Arc::new(SecretStore::new());
            let cfg = config.get();
            let hotkey = Arc::new(HotkeyService::new(&cfg.hotkey)?);
            let translator = Arc::new(Translator::new());

            let state = AppState {
                config: config.clone(),
                secrets: secrets.clone(),
                hotkey: hotkey.clone(),
                translator: translator.clone(),
                previous_window: std::sync::Arc::new(parking_lot::Mutex::new(None)),
            };
            app.manage(state);

            crate::tray::build_tray(&app.handle().clone())?;

            if let Some(w) = app.get_webview_window("popup") {
                w.hide().ok();
            }
            if let Some(w) = app.get_webview_window("settings") {
                w.hide().ok();
            }

            hotkey.start();

            let mut rx = hotkey.take_receiver().expect("receiver");
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Some(()) = rx.recv().await {
                    let h = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = crate::translator::run_from_selection(h).await {
                            log::warn!("hotkey translation failed: {e}");
                        }
                    });
                }
            });

            let args: Vec<String> = std::env::args().collect();
            let autostarted = args.iter().any(|a| a == "--autostart");
            if !autostarted {
                if let Some(w) = app.get_webview_window("settings") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }

            Ok(())
        })
        .on_window_event(|win, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let label = win.label();
                if label == "settings" || label == "popup" {
                    api.prevent_close();
                    let _ = win.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::list_languages,
            commands::get_api_key,
            commands::set_api_key,
            commands::clear_api_key,
            commands::fetch_models,
            commands::translate_text,
            commands::translate_selection,
            commands::cancel_translation,
            commands::close_popup,
            commands::pin_popup,
            commands::open_settings,
            commands::apply_hotkey,
            commands::apply_autostart,
            commands::test_provider,
            commands::swap_languages,
            commands::replace_selection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
