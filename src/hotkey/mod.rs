//! Local Windows-only port of the string-parsing parts of `handy-keys` v0.2.4 (MIT),
//! plus the keyboard hook state. Only the keyboard matters here; mouse entries parse
//! but never fire.

mod definition;
mod hook;
mod key;
mod modifiers;

pub use definition::Hotkey;
pub use hook::HotkeyListener;
pub use key::Key;
pub use modifiers::Modifiers;

/// Errors produced while parsing hotkeys or running the keyboard hook.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("hotkey must have at least one modifier or a key")]
    EmptyHotkey,
    #[error("invalid hotkey format: {0}")]
    InvalidFormat(String),
    #[error("unknown key: {0}")]
    UnknownKey(String),
    #[error("unknown modifier: {0}")]
    UnknownModifier(String),
    #[error("failed to install the low-level keyboard hook: {0}")]
    HookInstall(#[from] windows::core::Error),
    #[error("failed to spawn the keyboard hook thread: {0}")]
    ThreadSpawn(#[from] std::io::Error),
    #[error("a hotkey listener is already initialized in this process")]
    AlreadyInitialized,
    #[error("the keyboard hook thread closed before reporting readiness")]
    HookThreadClosed,
}
