use tauri::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

use crate::error::AppResult;
use crate::translator::run_from_selection;
use crate::windowing::show_settings;

pub fn build_tray(app: &AppHandle) -> AppResult<()> {
    let translate = MenuItem::with_id(app, "translate", "Translate selection", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings…", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&translate, &settings, &separator, &quit])?;

    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))
        .map_err(|e| crate::error::AppError::Other(format!("tray icon: {e}")))?;

    let _tray = TrayIconBuilder::with_id("main")
        .tooltip("WispLingua")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_menu(app, event))
        .on_tray_icon_event(|tray, event| handle_tray_event(tray.app_handle(), event))
        .build(app)?;

    Ok(())
}

fn handle_menu(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "translate" => {
            let handle = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = run_from_selection(handle).await {
                    log::warn!("translate: {e}");
                }
            });
        }
        "settings" => {
            let _ = show_settings(app);
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

fn handle_tray_event(app: &AppHandle, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        let _ = show_settings(app);
    }
}
