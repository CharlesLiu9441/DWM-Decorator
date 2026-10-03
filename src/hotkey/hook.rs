//! Global low-level keyboard hook (`WH_KEYBOARD_LL`) that tracks hotkey press state.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use tracing::warn;
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::Threading::GetCurrentThreadId,
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageW, GetMessageW, KBDLLHOOKSTRUCT, LLKHF_EXTENDED, MSG,
        PM_NOREMOVE, PeekMessageW, PostThreadMessageW, SetWindowsHookExW, TranslateMessage,
        UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_QUIT, WM_SYSKEYDOWN,
    },
};

use super::{Error, Hotkey, Modifiers, key::vk_to_key, modifiers::vk_to_modifier};

/// Shared state updated by the hook callback and read by the polling thread.
struct HookState {
    hotkeys: Vec<Hotkey>,
    intercept: bool,
    modifiers: AtomicU32,
    pressed: Vec<AtomicBool>,
}

impl HookState {
    fn new(hotkeys: &[Hotkey], intercept: bool) -> Self {
        Self {
            hotkeys: hotkeys.to_vec(),
            intercept,
            modifiers: AtomicU32::new(0),
            pressed: hotkeys.iter().map(|_| AtomicBool::new(false)).collect(),
        }
    }

    fn current_modifiers(&self) -> Modifiers {
        Modifiers::from_bits_truncate(self.modifiers.load(Ordering::SeqCst))
    }

    fn is_pressed(&self, index: usize) -> bool {
        self.pressed
            .get(index)
            .is_some_and(|flag| flag.load(Ordering::SeqCst))
    }

    /// Pure state transition for one keyboard event.
    ///
    /// Returns `true` when the hook must block the event, which requires
    /// intercept mode and a full hotkey match at event time.
    fn handle_event(&self, vk_code: u32, is_extended: bool, is_down: bool) -> bool {
        if let Some(modifier) = vk_to_modifier(vk_code) {
            let bits = modifier.bits();
            if is_down {
                self.modifiers.fetch_or(bits, Ordering::SeqCst);
            } else {
                self.modifiers.fetch_and(!bits, Ordering::SeqCst);
                // A released modifier can invalidate a held hotkey (for example
                // Ctrl+Numpad0 when Ctrl is released first). Clear those flags
                // using the post-release modifier state.
                let current = self.current_modifiers();
                for (index, hotkey) in self.hotkeys.iter().enumerate() {
                    if !hotkey.modifiers.matches(current)
                        && let Some(flag) = self.pressed.get(index)
                    {
                        flag.store(false, Ordering::SeqCst);
                    }
                }
            }
            // No modifier-only hotkeys exist: config guarantees a key is present.
            return false;
        }

        let Some(key) = vk_to_key(vk_code, is_extended) else {
            return false;
        };

        let current = self.current_modifiers();
        let mut should_block = false;
        for (index, hotkey) in self.hotkeys.iter().enumerate() {
            if hotkey.key != Some(key) {
                continue;
            }
            let Some(flag) = self.pressed.get(index) else {
                continue;
            };
            if is_down {
                if hotkey.modifiers.matches(current) {
                    flag.store(true, Ordering::SeqCst);
                    should_block |= self.intercept;
                }
            } else {
                // Always clear on keyup, even if the modifiers no longer match.
                flag.store(false, Ordering::SeqCst);
                should_block |= self.intercept && hotkey.modifiers.matches(current);
            }
        }
        should_block
    }
}

static STATE: OnceLock<HookState> = OnceLock::new();

/// Handle to the running keyboard hook thread.
///
/// Dropping the listener asks the hook thread to quit and joins it.
pub struct HotkeyListener {
    thread_id: u32,
    handle: Option<JoinHandle<()>>,
}

impl HotkeyListener {
    /// Install the global low-level keyboard hook and start its message thread.
    ///
    /// `hotkeys` order defines the indices accepted by [`Self::is_pressed`].
    /// Fails with [`Error::AlreadyInitialized`] if a listener already exists.
    pub fn new(hotkeys: &[Hotkey], intercept: bool) -> Result<Self, Error> {
        if STATE.get().is_some() {
            return Err(Error::AlreadyInitialized);
        }
        let state = HookState::new(hotkeys, intercept);
        let (ready_tx, ready_rx) = mpsc::channel::<Result<u32, windows::core::Error>>();
        let handle = thread::Builder::new()
            .name("keyboard-hook".to_string())
            .spawn(move || run_hook_thread(state, ready_tx))
            .map_err(Error::ThreadSpawn)?;
        match ready_rx.recv() {
            Ok(Ok(thread_id)) => Ok(Self {
                thread_id,
                handle: Some(handle),
            }),
            Ok(Err(error)) => {
                let _ = handle.join();
                Err(Error::HookInstall(error))
            }
            Err(_) => Err(Error::HookThreadClosed),
        }
    }

    /// Whether the hotkey at `index` (as passed to [`Self::new`]) is held down.
    pub fn is_pressed(&self, index: usize) -> bool {
        STATE.get().is_some_and(|state| state.is_pressed(index))
    }
}

impl Drop for HotkeyListener {
    fn drop(&mut self) {
        // SAFETY: `thread_id` was reported by the hook thread only after its
        // message queue had been created, so it identifies a valid queue.
        // Posting WM_QUIT to a valid queue is memory-safe, and the error is
        // intentionally ignored because there is no recovery during teardown.
        unsafe {
            let _ = PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn run_hook_thread(state: HookState, ready: mpsc::Sender<Result<u32, windows::core::Error>>) {
    let mut msg = MSG::default();

    // SAFETY: PeekMessageW with PM_NOREMOVE only inspects the calling thread's
    // message queue and removes nothing. `&mut msg` points to a valid `MSG`.
    unsafe {
        let _ = PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE);
    }

    // SAFETY: GetCurrentThreadId takes no arguments and simply returns the id
    // of the calling thread.
    let thread_id = unsafe { GetCurrentThreadId() };

    let hook = match unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0) }
    {
        Ok(hook) => hook,
        Err(error) => {
            let _ = ready.send(Err(error));
            return;
        }
    };

    if STATE.set(state).is_err() {
        warn!("Keyboard hook state was already initialized; press tracking is disabled");
    }

    if ready.send(Ok(thread_id)).is_err() {
        // The listener went away before readiness was reported; tear the hook
        // down instead of blocking forever in the message pump.
        // SAFETY: `hook` is a valid hook handle returned by SetWindowsHookExW.
        let _ = unsafe { UnhookWindowsHookEx(hook) };
        return;
    }

    // SAFETY: Standard Win32 message pump. GetMessageW blocks and writes a
    // valid `MSG` into `msg`; the loop terminates on WM_QUIT (return 0).
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        // SAFETY: `hook` is the valid handle returned by SetWindowsHookExW.
        let _ = UnhookWindowsHookEx(hook);
    }
}

unsafe extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: Calling CallNextHookEx with a null hook handle is permitted and
    // forwards the event to the next hook in the chain with the parameters we
    // received. This is required when code < 0.
    if code < 0 {
        return unsafe { CallNextHookEx(None, code, wparam, lparam) };
    }

    let Some(state) = STATE.get() else {
        // SAFETY: Forwarding the event unchanged to the next hook.
        return unsafe { CallNextHookEx(None, code, wparam, lparam) };
    };

    // SAFETY: For code >= 0 the Windows hook-proc contract guarantees that
    // `lparam` points to a valid, aligned KBDLLHOOKSTRUCT for the duration of
    // the call.
    let keyboard = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
    let message = u32::try_from(wparam.0).unwrap_or_default();
    let is_down = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);
    let is_extended = keyboard.flags.contains(LLKHF_EXTENDED);

    if state.handle_event(keyboard.vkCode, is_extended, is_down) {
        LRESULT(1)
    } else {
        // SAFETY: Forwarding the event unchanged to the next hook.
        unsafe { CallNextHookEx(None, code, wparam, lparam) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hotkey::Key;

    const VK_LCONTROL: u32 = 0xA2;
    const VK_LSHIFT: u32 = 0xA0;
    const VK_NUMPAD0: u32 = 0x60;
    const VK_NUMPAD2: u32 = 0x62;

    fn ctrl_keypad0() -> Hotkey {
        Hotkey::new(Modifiers::CTRL, Key::Keypad0).unwrap()
    }

    #[test]
    fn modifier_and_key_press_is_tracked() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        assert!(!state.handle_event(VK_LCONTROL, false, true));
        assert!(!state.handle_event(VK_NUMPAD0, false, true));
        assert!(state.is_pressed(0));
    }

    #[test]
    fn auto_repeat_keeps_pressed() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        state.handle_event(VK_LCONTROL, false, true);
        state.handle_event(VK_NUMPAD0, false, true);
        state.handle_event(VK_NUMPAD0, false, true);
        assert!(state.is_pressed(0));
    }

    #[test]
    fn key_release_clears_pressed() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        state.handle_event(VK_LCONTROL, false, true);
        state.handle_event(VK_NUMPAD0, false, true);
        state.handle_event(VK_NUMPAD0, false, false);
        assert!(!state.is_pressed(0));
    }

    #[test]
    fn modifier_release_clears_pressed() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        state.handle_event(VK_LCONTROL, false, true);
        state.handle_event(VK_NUMPAD0, false, true);
        assert!(state.is_pressed(0));
        state.handle_event(VK_LCONTROL, false, false);
        assert!(!state.is_pressed(0));
    }

    #[test]
    fn intercept_blocks_matching_down_and_up() {
        let state = HookState::new(&[ctrl_keypad0()], true);
        assert!(!state.handle_event(VK_LCONTROL, false, true));
        assert!(state.handle_event(VK_NUMPAD0, false, true));
        assert!(state.handle_event(VK_NUMPAD0, false, false));
    }

    #[test]
    fn listen_only_never_blocks() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        assert!(!state.handle_event(VK_LCONTROL, false, true));
        assert!(!state.handle_event(VK_NUMPAD0, false, true));
        assert!(!state.handle_event(VK_NUMPAD0, false, false));
    }

    #[test]
    fn missing_modifier_does_not_press() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        assert!(!state.handle_event(VK_NUMPAD0, false, true));
        assert!(!state.is_pressed(0));
    }

    #[test]
    fn wrong_modifier_does_not_press() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        state.handle_event(VK_LSHIFT, false, true);
        assert!(!state.handle_event(VK_NUMPAD0, false, true));
        assert!(!state.is_pressed(0));
    }

    #[test]
    fn unrelated_key_does_not_press() {
        let state = HookState::new(&[ctrl_keypad0()], false);
        state.handle_event(VK_LCONTROL, false, true);
        assert!(!state.handle_event(VK_NUMPAD2, false, true));
        assert!(!state.is_pressed(0));
    }
}
