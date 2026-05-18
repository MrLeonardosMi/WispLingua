use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};

use crate::hotkey::listener::CursorPos;

pub struct PopupPlacement {
    pub x: i32,
    pub y: i32,
}

pub fn place_popup(
    app: &AppHandle,
    cursor: CursorPos,
    popup_w: u32,
    popup_h: u32,
    prefer_below: bool,
) -> PopupPlacement {
    let monitors = app.available_monitors().unwrap_or_default();
    let target = monitors
        .iter()
        .find(|m| {
            let p = m.position();
            let s = m.size();
            cursor.x >= p.x
                && cursor.x < p.x + s.width as i32
                && cursor.y >= p.y
                && cursor.y < p.y + s.height as i32
        })
        .cloned()
        .or_else(|| monitors.first().cloned());

    let (mp, ms) = match target {
        Some(m) => (
            PhysicalPosition::<i32>::new(m.position().x, m.position().y),
            PhysicalSize::<u32>::new(m.size().width, m.size().height),
        ),
        None => (PhysicalPosition::new(0, 0), PhysicalSize::new(1920, 1080)),
    };

    let margin = 8_i32;
    let offset_x = 14_i32;
    let offset_y = 18_i32;

    let mut px = cursor.x + offset_x;
    let mut py = if prefer_below {
        cursor.y + offset_y
    } else {
        cursor.y - popup_h as i32 - offset_y
    };

    let max_x = mp.x + ms.width as i32 - popup_w as i32 - margin;
    let max_y = mp.y + ms.height as i32 - popup_h as i32 - margin;
    let min_x = mp.x + margin;
    let min_y = mp.y + margin;

    if prefer_below && py > max_y {
        let alt = cursor.y - popup_h as i32 - offset_y;
        if alt >= min_y {
            py = alt;
        } else {
            py = max_y;
        }
    } else if !prefer_below && py < min_y {
        let alt = cursor.y + offset_y;
        if alt <= max_y {
            py = alt;
        } else {
            py = min_y;
        }
    }

    if px > max_x { px = max_x; }
    if px < min_x { px = min_x; }
    if py > max_y { py = max_y; }
    if py < min_y { py = min_y; }

    PopupPlacement { x: px, y: py }
}
