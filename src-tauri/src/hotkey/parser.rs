use rdev::Key;

use crate::config::HotkeyTrigger;
use crate::error::{AppError, AppResult};

use super::chord::{Modifier, ParsedTrigger};

pub fn parse_trigger(t: &HotkeyTrigger) -> AppResult<ParsedTrigger> {
    match t {
        HotkeyTrigger::Combo { combo, presses } => {
            let (mods, key) = parse_combo(combo)?;
            Ok(ParsedTrigger::Combo {
                modifiers: mods,
                key,
                presses: (*presses).max(1),
            })
        }
        HotkeyTrigger::Modifier { modifier, presses } => {
            let m = parse_modifier(modifier)?;
            Ok(ParsedTrigger::ModifierTap { modifier: m, presses: (*presses).max(1) })
        }
    }
}

fn parse_combo(s: &str) -> AppResult<(Vec<Modifier>, Key)> {
    let parts: Vec<&str> = s.split('+').map(str::trim).filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        return Err(AppError::Hotkey(format!("empty combo: {s}")));
    }
    let (key_part, mod_parts) = parts.split_last().unwrap();
    let mut mods = Vec::new();
    for m in mod_parts {
        mods.push(parse_modifier(m)?);
    }
    let key = parse_key(key_part).ok_or_else(|| AppError::Hotkey(format!("unknown key: {key_part}")))?;
    Ok((mods, key))
}

pub fn parse_modifier(s: &str) -> AppResult<Modifier> {
    match s.to_ascii_lowercase().as_str() {
        "ctrl" | "control" => Ok(Modifier::Ctrl),
        "shift" => Ok(Modifier::Shift),
        "alt" | "option" => Ok(Modifier::Alt),
        "super" | "meta" | "cmd" | "win" | "windows" => Ok(Modifier::Super),
        other => Err(AppError::Hotkey(format!("unknown modifier: {other}"))),
    }
}

pub fn key_to_string(k: Key) -> String {
    format!("{k:?}")
}

#[allow(clippy::too_many_lines)]
pub fn parse_key(s: &str) -> Option<Key> {
    use Key::*;
    let upper = s.to_ascii_uppercase();
    if upper.len() == 1 {
        let c = upper.chars().next().unwrap();
        if c.is_ascii_alphabetic() {
            return letter_key(c);
        }
        if c.is_ascii_digit() {
            return digit_key(c);
        }
    }
    Some(match s {
        "Space" => Space,
        "Enter" | "Return" => Return,
        "Tab" => Tab,
        "Escape" | "Esc" => Escape,
        "Backspace" => Backspace,
        "Delete" => Delete,
        "Home" => Home,
        "End" => End,
        "PageUp" => PageUp,
        "PageDown" => PageDown,
        "Up" => UpArrow,
        "Down" => DownArrow,
        "Left" => LeftArrow,
        "Right" => RightArrow,
        "F1" => F1, "F2" => F2, "F3" => F3, "F4" => F4, "F5" => F5,
        "F6" => F6, "F7" => F7, "F8" => F8, "F9" => F9, "F10" => F10,
        "F11" => F11, "F12" => F12,
        "Backquote" | "`" => BackQuote,
        "Minus" | "-" => Minus,
        "Equal" | "=" => Equal,
        "BracketLeft" | "[" => LeftBracket,
        "BracketRight" | "]" => RightBracket,
        "Backslash" | "\\" => BackSlash,
        "Semicolon" | ";" => SemiColon,
        "Quote" | "'" => Quote,
        "Comma" | "," => Comma,
        "Period" | "." => Dot,
        "Slash" | "/" => Slash,
        _ => return None,
    })
}

fn letter_key(c: char) -> Option<Key> {
    use Key::*;
    Some(match c {
        'A' => KeyA, 'B' => KeyB, 'C' => KeyC, 'D' => KeyD, 'E' => KeyE,
        'F' => KeyF, 'G' => KeyG, 'H' => KeyH, 'I' => KeyI, 'J' => KeyJ,
        'K' => KeyK, 'L' => KeyL, 'M' => KeyM, 'N' => KeyN, 'O' => KeyO,
        'P' => KeyP, 'Q' => KeyQ, 'R' => KeyR, 'S' => KeyS, 'T' => KeyT,
        'U' => KeyU, 'V' => KeyV, 'W' => KeyW, 'X' => KeyX, 'Y' => KeyY, 'Z' => KeyZ,
        _ => return None,
    })
}

fn digit_key(c: char) -> Option<Key> {
    use Key::*;
    Some(match c {
        '0' => Num0, '1' => Num1, '2' => Num2, '3' => Num3, '4' => Num4,
        '5' => Num5, '6' => Num6, '7' => Num7, '8' => Num8, '9' => Num9,
        _ => return None,
    })
}
