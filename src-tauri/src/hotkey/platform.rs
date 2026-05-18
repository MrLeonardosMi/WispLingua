use super::chord::Modifier;

#[cfg(target_os = "windows")]
pub fn modifier_held(m: Modifier) -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
    };
    let vks: &[u16] = match m {
        Modifier::Ctrl => &[VK_CONTROL.0],
        Modifier::Shift => &[VK_SHIFT.0],
        Modifier::Alt => &[VK_MENU.0],
        Modifier::Super => &[VK_LWIN.0, VK_RWIN.0],
    };
    vks.iter().any(|vk| {
        let state = unsafe { GetAsyncKeyState(*vk as i32) };
        (state as u16) & 0x8000 != 0
    })
}

#[cfg(not(target_os = "windows"))]
pub fn modifier_held(_m: Modifier) -> bool {
    false
}

#[cfg(target_os = "windows")]
pub const HAS_OS_MODIFIER_QUERY: bool = true;

#[cfg(not(target_os = "windows"))]
pub const HAS_OS_MODIFIER_QUERY: bool = false;
