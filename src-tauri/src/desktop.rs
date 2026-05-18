#[cfg(target_os = "windows")]
pub fn current_foreground() -> Option<isize> {
    use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    let hwnd = unsafe { GetForegroundWindow() };
    let raw = hwnd.0 as isize;
    if raw == 0 {
        None
    } else {
        Some(raw)
    }
}

#[cfg(target_os = "windows")]
pub fn focus(hwnd_id: isize) -> bool {
    if hwnd_id == 0 {
        return false;
    }
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        keybd_event, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_MENU,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        IsIconic, SetForegroundWindow, ShowWindow, SW_RESTORE,
    };

    let hwnd = HWND(hwnd_id as *mut _);
    unsafe {
        keybd_event(VK_MENU.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }
        SetForegroundWindow(hwnd).as_bool()
    }
}

#[cfg(not(target_os = "windows"))]
pub fn current_foreground() -> Option<isize> {
    None
}

#[cfg(not(target_os = "windows"))]
pub fn focus(_hwnd_id: isize) -> bool {
    false
}
