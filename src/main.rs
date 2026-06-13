#![windows_subsystem = "windows"]
mod configuration;
// TODO: add key configuration
// TODO: add log system

use handy_keys::{Hotkey, HotkeyManager, HotkeyState};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::mpsc::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::*;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Accessibility::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum KeyAction {
    Topmost,
    IncreaseTransparency,
    DecreaseTransparency,
}
#[derive(Clone, Copy)]
struct SendHWND(HWND);
unsafe impl Send for SendHWND {}
static TX: OnceLock<Sender<(SendHWND, SendHWND)>> = OnceLock::new();
static CONFIG: OnceLock<configuration::DecodedConfig> = OnceLock::new();

fn main() -> handy_keys::Result<()> {
    CONFIG.set(configuration::load_config()).unwrap_or(());
    unsafe {
        SetPriorityClass(GetCurrentProcess(), HIGH_PRIORITY_CLASS).unwrap_or(());
    }
    let (tx, rx) = channel::<(SendHWND, SendHWND)>();
    TX.set(tx).unwrap_or(());
    std::thread::spawn(move || {
        let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        let mut last_focus: Option<HWND> = None;
        while let Ok(send_hwnd) = rx.recv() {
            proc_hwnd(send_hwnd.0.0, send_hwnd.1.0, &mut last_focus);
        }
    });
    unsafe {
        let _ = EnumWindows(Some(enum_window_proc), LPARAM(0));
        let _ = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
    }
    let manager = HotkeyManager::new()?;
    let mut action_map = HashMap::new();

    let topmost: Hotkey = "Ctrl+F10".parse()?;
    action_map.insert(manager.register(topmost.clone())?, KeyAction::Topmost);

    let increase_transparency: Hotkey = "Ctrl+F11".parse()?;
    action_map.insert(
        manager.register(increase_transparency.clone())?,
        KeyAction::IncreaseTransparency,
    );

    let decrease_transparency: Hotkey = "Ctrl+F12".parse()?;
    action_map.insert(
        manager.register(decrease_transparency.clone())?,
        KeyAction::DecreaseTransparency,

    );
    std::thread::spawn(move || {
        while let Ok(event) = manager.recv() {
            if let Some(action) = action_map.get(&event.id) && event.state==HotkeyState::Pressed {
                match action {
                    KeyAction::Topmost => unsafe {
                        let top_hwnd = GetForegroundWindow();
                        let current_style = GetWindowLongPtrW(top_hwnd, GWL_EXSTYLE);
                        let new_style = current_style ^ (WS_EX_TOPMOST.0 as isize);
                        let is_now_topmost = (new_style & (WS_EX_TOPMOST.0 as isize)) != 0;
                        let insert_after = if is_now_topmost {
                            HWND_TOPMOST
                        } else {
                            HWND_NOTOPMOST
                        };
                        SetWindowLongPtrW(top_hwnd, GWL_EXSTYLE, new_style);
                        SetWindowPos(
                            top_hwnd,
                            Some(insert_after),
                            0,
                            0,
                            0,
                            0,
                            SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED,
                        )
                        .unwrap_or(());
                        if let Some(tx) = TX.get() {
                            let _ = tx.send((SendHWND(top_hwnd), SendHWND(top_hwnd)));
                        }
                    },
                    KeyAction::IncreaseTransparency | KeyAction::DecreaseTransparency => unsafe {
                        let top_hwnd = GetForegroundWindow();
                        let mut ex_style = GetWindowLongPtrW(top_hwnd, GWL_EXSTYLE);
                        if (ex_style & WS_EX_LAYERED.0 as isize) == 0 {
                            ex_style |= WS_EX_LAYERED.0 as isize;
                            SetWindowLongPtrW(top_hwnd, GWL_EXSTYLE, ex_style);
                            let _ =
                                SetLayeredWindowAttributes(top_hwnd, COLORREF(0), 255, LWA_ALPHA);
                        }
                        let mut alpha: u8 = 0;
                        let mut flags: LAYERED_WINDOW_ATTRIBUTES_FLAGS =
                            LAYERED_WINDOW_ATTRIBUTES_FLAGS(0);
                        let mut color_key = COLORREF(0);
                        if GetLayeredWindowAttributes(
                            top_hwnd,
                            Some(&mut color_key),
                            Some(&mut alpha),
                            Some(&mut flags),
                        )
                        .is_err()
                        {
                            alpha = 255;
                        }
                        let new_alpha = if *action == KeyAction::IncreaseTransparency {
                            (alpha as f64*0.9).max(15.0).round() as u8
                        } else {
                            (alpha as f64/0.9).min(255.0).round() as u8
                        };
                        let _ = SetLayeredWindowAttributes(
                            top_hwnd,
                            color_key,
                            new_alpha,
                            flags | LWA_ALPHA,
                        );
                    },
                }
            }
        }
    });
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}
extern "system" fn enum_window_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    if let Some(tx) = TX.get() {
        let _ = tx.send((SendHWND(hwnd), SendHWND(unsafe { GetForegroundWindow() })));
    }
    true.into()
}
unsafe extern "system" fn win_event_proc(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dw_ms_event_time: u32,
) {
    if id_object == OBJID_WINDOW.0 && !hwnd.is_invalid() {
        if let Some(tx) = TX.get() {
            let _ = tx.send((SendHWND(hwnd), SendHWND(unsafe { GetForegroundWindow() })));
        }
    }
}
fn is_topmost(hwnd: &HWND) -> bool {
    unsafe {
        let style = GetWindowLongPtrW(*hwnd, GWL_EXSTYLE) as u32;
        (WINDOW_EX_STYLE(style) & WS_EX_TOPMOST).0 != 0
    }
}
fn proc_hwnd(hwnd: HWND, foreground: HWND, last_focus: &mut Option<HWND>) {
    let default_config = configuration::DecodedConfig::default();
    let config = CONFIG.get().unwrap_or(&default_config);
    unsafe {
        if config.force_color_scheme >= 0 {
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &config.force_color_scheme as *const i32 as *const _,
                size_of::<i32>() as u32,
            )
            .unwrap_or(());
        }
        if config.force_border_radius >= 0 {
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_WINDOW_CORNER_PREFERENCE,
                &config.force_border_radius as *const i32 as *const _,
                size_of::<i32>() as u32,
            )
            .unwrap_or(());
        }
        let color;
        if hwnd == foreground {
            color = if is_topmost(&hwnd) {
                config.active_topmost_border_color
            } else {
                config.active_border_color
            };
            if Some(hwnd) != *last_focus {
                if let Some(last_focus) = last_focus {
                    if !last_focus.is_invalid() {
                        let _ = DwmSetWindowAttribute(
                            *last_focus,
                            DWMWA_TEXT_COLOR,
                            &COLORREF(0xcfc8c0) as *const COLORREF as *const _,
                            size_of::<COLORREF>() as u32,
                        );
                        let _ = DwmSetWindowAttribute(
                            *last_focus,
                            DWMWA_BORDER_COLOR,
                            &if is_topmost(last_focus) {
                                config.inactive_topmost_border_color
                            } else {
                                config.inactive_border_color
                            } as *const COLORREF as *const _,
                            size_of::<COLORREF>() as u32,
                        );
                    }
                }
                *last_focus = Some(hwnd);
            }
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_TEXT_COLOR,
                &COLORREF(0xfff8f0) as *const COLORREF as *const _,
                size_of::<COLORREF>() as u32,
            );
        } else {
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_TEXT_COLOR,
                &COLORREF(0xcfc8c0) as *const COLORREF as *const _,
                size_of::<COLORREF>() as u32,
            );
            color = if is_topmost(&hwnd) {
                config.inactive_topmost_border_color
            } else {
                config.inactive_border_color
            };
        }
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_BORDER_COLOR,
            &color as *const COLORREF as *const _,
            size_of::<COLORREF>() as u32,
        );
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_CAPTION_COLOR,
            &COLORREF(0x202020) as *const COLORREF as *const _,
            size_of::<COLORREF>() as u32,
        );
    }
}
