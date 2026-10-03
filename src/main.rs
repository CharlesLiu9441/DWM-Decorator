#![windows_subsystem = "windows"]
mod configuration;
mod hotkey;
pub mod logger;

use hotkey::HotkeyListener;
use std::sync::{OnceLock, mpsc::*};
use tracing::{error, info, warn};
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Dwm::*,
        System::{
            Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize},
            Threading::*,
        },
        UI::{Accessibility::*, WindowsAndMessaging::*},
    },
    core::*,
};
#[derive(Clone, Copy)]
struct SendHWND(HWND);
unsafe impl Send for SendHWND {}
static TX: OnceLock<Sender<(SendHWND, SendHWND)>> = OnceLock::new();
static CONFIG: OnceLock<configuration::DecodedConfig> = OnceLock::new();
const IDX_TOPMOST: usize = 0;
const IDX_INCREASE_TRANSPARENCY: usize = 1;
const IDX_DECREASE_TRANSPARENCY: usize = 2;

fn main() {
    let guard = logger::init_logger();
    logger::setup_panic_hook();
    let config = configuration::load_config();
    CONFIG.set(config).expect("CONFIG setting failed");
    unsafe {
        SetPriorityClass(GetCurrentProcess(), HIGH_PRIORITY_CLASS).unwrap_or_else(|error| {
            warn!(%error, "Failed to set high priority class...");
        });
    }
    let (tx, rx) = channel::<(SendHWND, SendHWND)>();
    TX.set(tx).expect("TX setting failed");
    unsafe {
        EnumWindows(Some(enum_window_proc), LPARAM(0))
            .unwrap_or_else(|error| error!(%error,"EnumWindows failed"));
    }
    let win_event_hook = unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    };
    let hotkeys = [
        config.key_toggle_topmost,
        config.key_increase_transparency,
        config.key_decrease_transparency,
    ];
    match HotkeyListener::new(&hotkeys, config.intercept_hotkeys) {
        Ok(listener) => {
            info!(
                ?hotkeys,
                intercept = config.intercept_hotkeys,
                "Keyboard hotkeys registered"
            );
            std::thread::spawn(move || run_hotkey_loop(listener));
        }
        Err(error) => {
            error!(%error, "Failed to install the keyboard hook; hotkeys are disabled");
        }
    }
    std::thread::spawn(move || {
        let mut need_uninitialize = false;
        match unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) } {
            S_OK => {
                need_uninitialize = true;
                info!("COM library initialized successfully for this thread.");
            }
            S_FALSE => {
                need_uninitialize = true;
                warn!(
                    "COM library was already initialized on this thread with the same mode. (S_FALSE)"
                );
            }
            RPC_E_CHANGED_MODE => {
                warn!(
                    error = "RPC_E_CHANGED_MODE",
                    "COM library initialization failed: Concurrency mode conflict. Another component already initialized this thread as MTA. Proceeding with caution."
                );
            }
            other_err => {
                error!(
                    error_code = ?other_err,
                    "FATAL: CoInitializeEx failed catastrophically."
                );
            }
        }
        let mut last_focus: Option<HWND> = None;
        while let Ok(send_hwnd) = rx.recv() {
            proc_hwnd(send_hwnd.0.0, send_hwnd.1.0, &mut last_focus);
        }
        if need_uninitialize {
            unsafe { CoUninitialize() }
        }
    });
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let _ = UnhookWinEvent(win_event_hook);
    }
    drop(guard);
}
fn run_hotkey_loop(listener: HotkeyListener) {
    let mut topmost_was_pressed = false;
    let mut increase_was_pressed = false;
    let mut decrease_was_pressed = false;
    let mut press_duration: u32 = 0;
    loop {
        let topmost_pressed = listener.is_pressed(IDX_TOPMOST);
        if topmost_pressed && !topmost_was_pressed {
            info!(action = "toggle_topmost", "Hotkey pressed");
            toggle_topmost();
        }
        topmost_was_pressed = topmost_pressed;

        let increase_pressed = listener.is_pressed(IDX_INCREASE_TRANSPARENCY);
        if increase_pressed && !increase_was_pressed {
            info!(action = "increase_transparency", "Hotkey pressed");
        }
        increase_was_pressed = increase_pressed;

        let decrease_pressed = listener.is_pressed(IDX_DECREASE_TRANSPARENCY);
        if decrease_pressed && !decrease_was_pressed {
            info!(action = "decrease_transparency", "Hotkey pressed");
        }
        decrease_was_pressed = decrease_pressed;

        let direction = match (increase_pressed, decrease_pressed) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        if direction == 0 {
            press_duration = 0;
        } else {
            press_duration = (press_duration + 1).clamp(0, 1000);
            change_alpha(direction * ((press_duration as f64).ln().max(1.0).round() as i32));
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
fn change_alpha(delta: i32) {
    unsafe {
        let top_hwnd = GetForegroundWindow();
        let mut ex_style = GetWindowLongPtrW(top_hwnd, GWL_EXSTYLE);
        if (ex_style & WS_EX_LAYERED.0 as isize) == 0 {
            ex_style |= WS_EX_LAYERED.0 as isize;
            SetWindowLongPtrW(top_hwnd, GWL_EXSTYLE, ex_style);
            SetWindowPos(
                top_hwnd,
                None,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED | SWP_NOACTIVATE,
            )
            .unwrap_or_else(|error| warn!(%error,"fail to refresh window"));
            SetLayeredWindowAttributes(top_hwnd, COLORREF(0), 255, LWA_ALPHA).unwrap_or_else(
                |error| warn!(%error,"Error changing window into layered window attributes."),
            );
        }
        let mut alpha: u8 = 0;
        let mut flags: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(0);
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
        let new_alpha = (alpha as i32 + delta).clamp(15, 255) as u8;
        SetLayeredWindowAttributes(top_hwnd, color_key, new_alpha, flags | LWA_ALPHA)
            .unwrap_or_else(|error| warn!(%error,"Error setting window alpha"));
    }
}
fn toggle_topmost() {
    unsafe {
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
        .unwrap_or_else(|error| warn!(%error, "Error setting window position"));
        send_window(top_hwnd, top_hwnd);
    }
}
extern "system" fn enum_window_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    if unsafe { IsWindowVisible(hwnd).as_bool() } {
        send_window(hwnd, unsafe { GetForegroundWindow() });
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
        send_window(hwnd, unsafe { GetForegroundWindow() });
    }
}
fn send_window(a: HWND, b: HWND) {
    if let Some(tx) = TX.get() {
        tx.send((SendHWND(a), SendHWND(b))).unwrap_or_else(|error| {
            error!(%error, "Error sending message");
        });
    } else {
        error!("Error getting tx");
    }
}
fn is_topmost(hwnd: &HWND) -> bool {
    unsafe {
        let style = GetWindowLongPtrW(*hwnd, GWL_EXSTYLE) as u32;
        (WINDOW_EX_STYLE(style) & WS_EX_TOPMOST).0 != 0
    }
}
fn set_attr(h: HWND, attr: DWMWINDOWATTRIBUTE, val: &impl Copy) {
    if h.is_invalid() {
        return;
    }
    unsafe {
        DwmSetWindowAttribute(
            h,
            attr,
            val as *const _ as *const _,
            size_of_val(val) as u32,
        )
        .unwrap_or_else(|error| error!(%error, ?attr, "Error setting attribute"));
    }
}
fn proc_hwnd(hwnd: HWND, foreground: HWND, last_focus: &mut Option<HWND>) {
    let default_config = configuration::DecodedConfig::default();
    let config = match CONFIG.get() {
        Some(config) => config,
        None => {
            error!("Fail to get global configuration.");
            &default_config
        }
    };
    if config.force_color_scheme >= 0 {
        set_attr(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &config.force_color_scheme,
        );
    }
    if config.force_border_radius >= 0 {
        set_attr(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &config.force_border_radius,
        );
    }
    if hwnd == foreground && Some(hwnd) != *last_focus {
        if let Some(old_focus) = last_focus.filter(|h| !h.is_invalid()) {
            let old_border = if is_topmost(&old_focus) {
                config.inactive_topmost_border_color
            } else {
                config.inactive_border_color
            };
            set_attr(old_focus, DWMWA_TEXT_COLOR, &config.inactive_text_color);
            set_attr(old_focus, DWMWA_BORDER_COLOR, &old_border);
        }
        *last_focus = Some(hwnd);
    }
    let is_active = hwnd == foreground;
    let text_color = if is_active {
        config.active_text_color
    } else {
        config.inactive_text_color
    };
    let captain_color = if is_active {
        config.active_title_color
    } else {
        config.inactive_title_color
    };

    let border_color = match (is_active, is_topmost(&hwnd)) {
        (true, true) => config.active_topmost_border_color,
        (true, false) => config.active_border_color,
        (false, true) => config.inactive_topmost_border_color,
        (false, false) => config.inactive_border_color,
    };
    set_attr(hwnd, DWMWA_BORDER_COLOR, &border_color);
    if let Some(text_color) = text_color {
        set_attr(hwnd, DWMWA_TEXT_COLOR, &text_color);
    }
    if let Some(captain_color) = captain_color {
        set_attr(hwnd, DWMWA_CAPTION_COLOR, &captain_color);
    }
}
