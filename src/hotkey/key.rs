//! Keyboard key definitions, string parsing, and Windows virtual-key mapping.
//!
//! Ported from the `handy-keys` crate v0.2.4 (MIT), `types/key.rs` and
//! `platform/windows/keycode.rs`.

use std::fmt;
use std::str::FromStr;

use crate::hotkey::Error;

// allow: SIZE_OK — pure data tables ported from handy-keys

/// Keyboard keys and mouse buttons that can be used in hotkey combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,

    // Special keys
    Space,
    Return,
    Tab,
    Escape,
    Delete,
    ForwardDelete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,

    // Arrow keys
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    // Punctuation and symbols
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Grave,
    Section,
    // JIS keyboard keys
    JisYen,
    JisUnderscore,
    JisEisu,
    JisKana,

    // Keypad
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadDecimal,
    KeypadMultiply,
    KeypadPlus,
    KeypadClear,
    KeypadDivide,
    KeypadEnter,
    KeypadMinus,
    KeypadEquals,
    KeypadComma,

    // Lock keys
    CapsLock,
    ScrollLock,
    NumLock,

    // Mouse buttons
    MouseLeft,
    MouseRight,
    MouseMiddle,
    /// Extra button 1 (often "back" on mice with side buttons)
    MouseX1,
    /// Extra button 2 (often "forward" on mice with side buttons)
    MouseX2,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::A => write!(f, "A"),
            Key::B => write!(f, "B"),
            Key::C => write!(f, "C"),
            Key::D => write!(f, "D"),
            Key::E => write!(f, "E"),
            Key::F => write!(f, "F"),
            Key::G => write!(f, "G"),
            Key::H => write!(f, "H"),
            Key::I => write!(f, "I"),
            Key::J => write!(f, "J"),
            Key::K => write!(f, "K"),
            Key::L => write!(f, "L"),
            Key::M => write!(f, "M"),
            Key::N => write!(f, "N"),
            Key::O => write!(f, "O"),
            Key::P => write!(f, "P"),
            Key::Q => write!(f, "Q"),
            Key::R => write!(f, "R"),
            Key::S => write!(f, "S"),
            Key::T => write!(f, "T"),
            Key::U => write!(f, "U"),
            Key::V => write!(f, "V"),
            Key::W => write!(f, "W"),
            Key::X => write!(f, "X"),
            Key::Y => write!(f, "Y"),
            Key::Z => write!(f, "Z"),
            Key::Num0 => write!(f, "0"),
            Key::Num1 => write!(f, "1"),
            Key::Num2 => write!(f, "2"),
            Key::Num3 => write!(f, "3"),
            Key::Num4 => write!(f, "4"),
            Key::Num5 => write!(f, "5"),
            Key::Num6 => write!(f, "6"),
            Key::Num7 => write!(f, "7"),
            Key::Num8 => write!(f, "8"),
            Key::Num9 => write!(f, "9"),
            Key::F1 => write!(f, "F1"),
            Key::F2 => write!(f, "F2"),
            Key::F3 => write!(f, "F3"),
            Key::F4 => write!(f, "F4"),
            Key::F5 => write!(f, "F5"),
            Key::F6 => write!(f, "F6"),
            Key::F7 => write!(f, "F7"),
            Key::F8 => write!(f, "F8"),
            Key::F9 => write!(f, "F9"),
            Key::F10 => write!(f, "F10"),
            Key::F11 => write!(f, "F11"),
            Key::F12 => write!(f, "F12"),
            Key::F13 => write!(f, "F13"),
            Key::F14 => write!(f, "F14"),
            Key::F15 => write!(f, "F15"),
            Key::F16 => write!(f, "F16"),
            Key::F17 => write!(f, "F17"),
            Key::F18 => write!(f, "F18"),
            Key::F19 => write!(f, "F19"),
            Key::F20 => write!(f, "F20"),
            Key::Space => write!(f, "Space"),
            Key::Return => write!(f, "Return"),
            Key::Tab => write!(f, "Tab"),
            Key::Escape => write!(f, "Escape"),
            Key::Delete => write!(f, "Delete"),
            Key::ForwardDelete => write!(f, "ForwardDelete"),
            Key::Insert => write!(f, "Insert"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PageUp"),
            Key::PageDown => write!(f, "PageDown"),
            Key::LeftArrow => write!(f, "Left"),
            Key::RightArrow => write!(f, "Right"),
            Key::UpArrow => write!(f, "Up"),
            Key::DownArrow => write!(f, "Down"),
            Key::Minus => write!(f, "-"),
            Key::Equal => write!(f, "="),
            Key::LeftBracket => write!(f, "["),
            Key::RightBracket => write!(f, "]"),
            Key::Backslash => write!(f, "\\"),
            Key::Semicolon => write!(f, ";"),
            Key::Quote => write!(f, "'"),
            Key::Comma => write!(f, ","),
            Key::Period => write!(f, "."),
            Key::Slash => write!(f, "/"),
            Key::Grave => write!(f, "`"),
            Key::Section => write!(f, "§"),
            Key::JisYen => write!(f, "¥"),
            Key::JisUnderscore => write!(f, "JisUnderscore"),
            Key::JisEisu => write!(f, "Eisu"),
            Key::JisKana => write!(f, "Kana"),
            Key::Keypad0 => write!(f, "Keypad0"),
            Key::Keypad1 => write!(f, "Keypad1"),
            Key::Keypad2 => write!(f, "Keypad2"),
            Key::Keypad3 => write!(f, "Keypad3"),
            Key::Keypad4 => write!(f, "Keypad4"),
            Key::Keypad5 => write!(f, "Keypad5"),
            Key::Keypad6 => write!(f, "Keypad6"),
            Key::Keypad7 => write!(f, "Keypad7"),
            Key::Keypad8 => write!(f, "Keypad8"),
            Key::Keypad9 => write!(f, "Keypad9"),
            Key::KeypadDecimal => write!(f, "KeypadDecimal"),
            Key::KeypadMultiply => write!(f, "KeypadMultiply"),
            Key::KeypadPlus => write!(f, "KeypadPlus"),
            Key::KeypadClear => write!(f, "KeypadClear"),
            Key::KeypadDivide => write!(f, "KeypadDivide"),
            Key::KeypadEnter => write!(f, "KeypadEnter"),
            Key::KeypadMinus => write!(f, "KeypadMinus"),
            Key::KeypadEquals => write!(f, "KeypadEquals"),
            Key::KeypadComma => write!(f, "KeypadComma"),
            Key::CapsLock => write!(f, "CapsLock"),
            Key::ScrollLock => write!(f, "ScrollLock"),
            Key::NumLock => write!(f, "NumLock"),
            Key::MouseLeft => write!(f, "MouseLeft"),
            Key::MouseRight => write!(f, "MouseRight"),
            Key::MouseMiddle => write!(f, "MouseMiddle"),
            Key::MouseX1 => write!(f, "MouseX1"),
            Key::MouseX2 => write!(f, "MouseX2"),
        }
    }
}

/// All key aliases, lower-cased exactly where the source matched them.
///
/// The canonical `Display` string of every variant appears here, so
/// `format!("{key}").parse::<Key>()` round-trips for every key.
const ALIASES: &[(&str, Key)] = &[
    // Letters
    ("a", Key::A),
    ("b", Key::B),
    ("c", Key::C),
    ("d", Key::D),
    ("e", Key::E),
    ("f", Key::F),
    ("g", Key::G),
    ("h", Key::H),
    ("i", Key::I),
    ("j", Key::J),
    ("k", Key::K),
    ("l", Key::L),
    ("m", Key::M),
    ("n", Key::N),
    ("o", Key::O),
    ("p", Key::P),
    ("q", Key::Q),
    ("r", Key::R),
    ("s", Key::S),
    ("t", Key::T),
    ("u", Key::U),
    ("v", Key::V),
    ("w", Key::W),
    ("x", Key::X),
    ("y", Key::Y),
    ("z", Key::Z),
    // Numbers
    ("0", Key::Num0),
    ("num0", Key::Num0),
    ("1", Key::Num1),
    ("num1", Key::Num1),
    ("2", Key::Num2),
    ("num2", Key::Num2),
    ("3", Key::Num3),
    ("num3", Key::Num3),
    ("4", Key::Num4),
    ("num4", Key::Num4),
    ("5", Key::Num5),
    ("num5", Key::Num5),
    ("6", Key::Num6),
    ("num6", Key::Num6),
    ("7", Key::Num7),
    ("num7", Key::Num7),
    ("8", Key::Num8),
    ("num8", Key::Num8),
    ("9", Key::Num9),
    ("num9", Key::Num9),
    // Function keys
    ("f1", Key::F1),
    ("f2", Key::F2),
    ("f3", Key::F3),
    ("f4", Key::F4),
    ("f5", Key::F5),
    ("f6", Key::F6),
    ("f7", Key::F7),
    ("f8", Key::F8),
    ("f9", Key::F9),
    ("f10", Key::F10),
    ("f11", Key::F11),
    ("f12", Key::F12),
    ("f13", Key::F13),
    ("f14", Key::F14),
    ("f15", Key::F15),
    ("f16", Key::F16),
    ("f17", Key::F17),
    ("f18", Key::F18),
    ("f19", Key::F19),
    ("f20", Key::F20),
    // Special keys
    ("space", Key::Space),
    (" ", Key::Space),
    ("return", Key::Return),
    ("enter", Key::Return),
    ("tab", Key::Tab),
    ("escape", Key::Escape),
    ("esc", Key::Escape),
    ("delete", Key::Delete),
    ("backspace", Key::Delete),
    ("forwarddelete", Key::ForwardDelete),
    ("del", Key::ForwardDelete),
    ("insert", Key::Insert),
    ("ins", Key::Insert),
    ("home", Key::Home),
    ("end", Key::End),
    ("pageup", Key::PageUp),
    ("pagedown", Key::PageDown),
    // Arrow keys
    ("left", Key::LeftArrow),
    ("leftarrow", Key::LeftArrow),
    ("right", Key::RightArrow),
    ("rightarrow", Key::RightArrow),
    ("up", Key::UpArrow),
    ("uparrow", Key::UpArrow),
    ("down", Key::DownArrow),
    ("downarrow", Key::DownArrow),
    // Punctuation and symbols
    ("-", Key::Minus),
    ("minus", Key::Minus),
    ("=", Key::Equal),
    ("equal", Key::Equal),
    ("equals", Key::Equal),
    ("[", Key::LeftBracket),
    ("leftbracket", Key::LeftBracket),
    ("]", Key::RightBracket),
    ("rightbracket", Key::RightBracket),
    ("\\", Key::Backslash),
    ("backslash", Key::Backslash),
    (";", Key::Semicolon),
    ("semicolon", Key::Semicolon),
    ("'", Key::Quote),
    ("quote", Key::Quote),
    (",", Key::Comma),
    ("comma", Key::Comma),
    (".", Key::Period),
    ("period", Key::Period),
    ("/", Key::Slash),
    ("slash", Key::Slash),
    ("`", Key::Grave),
    ("grave", Key::Grave),
    ("backtick", Key::Grave),
    ("§", Key::Section),
    ("section", Key::Section),
    ("¥", Key::JisYen),
    ("jisyen", Key::JisYen),
    ("yen", Key::JisYen),
    ("jisunderscore", Key::JisUnderscore),
    ("eisu", Key::JisEisu),
    ("jiseisu", Key::JisEisu),
    ("英数", Key::JisEisu),
    ("kana", Key::JisKana),
    ("jiskana", Key::JisKana),
    ("かな", Key::JisKana),
    // Keypad
    ("keypad0", Key::Keypad0),
    ("keypad1", Key::Keypad1),
    ("keypad2", Key::Keypad2),
    ("keypad3", Key::Keypad3),
    ("keypad4", Key::Keypad4),
    ("keypad5", Key::Keypad5),
    ("keypad6", Key::Keypad6),
    ("keypad7", Key::Keypad7),
    ("keypad8", Key::Keypad8),
    ("keypad9", Key::Keypad9),
    ("keypad.", Key::KeypadDecimal),
    ("keypaddecimal", Key::KeypadDecimal),
    ("keypad*", Key::KeypadMultiply),
    ("keypadmultiply", Key::KeypadMultiply),
    ("keypad+", Key::KeypadPlus),
    ("keypadplus", Key::KeypadPlus),
    ("keypadclear", Key::KeypadClear),
    ("keypad/", Key::KeypadDivide),
    ("keypaddivide", Key::KeypadDivide),
    ("keypadenter", Key::KeypadEnter),
    ("keypad-", Key::KeypadMinus),
    ("keypadminus", Key::KeypadMinus),
    ("keypad=", Key::KeypadEquals),
    ("keypadequals", Key::KeypadEquals),
    ("keypad,", Key::KeypadComma),
    ("keypadcomma", Key::KeypadComma),
    // Lock keys
    ("capslock", Key::CapsLock),
    ("caps", Key::CapsLock),
    ("scrolllock", Key::ScrollLock),
    ("scroll", Key::ScrollLock),
    ("numlock", Key::NumLock),
    // Mouse buttons
    ("mouseleft", Key::MouseLeft),
    ("leftclick", Key::MouseLeft),
    ("lmb", Key::MouseLeft),
    ("mouse1", Key::MouseLeft),
    ("mouseright", Key::MouseRight),
    ("rightclick", Key::MouseRight),
    ("rmb", Key::MouseRight),
    ("mouse2", Key::MouseRight),
    ("mousemiddle", Key::MouseMiddle),
    ("middleclick", Key::MouseMiddle),
    ("mmb", Key::MouseMiddle),
    ("mouse3", Key::MouseMiddle),
    ("mousex1", Key::MouseX1),
    ("mouse4", Key::MouseX1),
    ("back", Key::MouseX1),
    ("xbutton1", Key::MouseX1),
    ("mousex2", Key::MouseX2),
    ("mouse5", Key::MouseX2),
    ("forward", Key::MouseX2),
    ("xbutton2", Key::MouseX2),
];

impl FromStr for Key {
    type Err = Error;

    /// Parse a key from its string representation (case-insensitive).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        ALIASES
            .iter()
            .find(|(alias, _)| alias.eq_ignore_ascii_case(s))
            .map(|(_, key)| *key)
            .ok_or_else(|| Error::UnknownKey(s.to_string()))
    }
}

/// Windows virtual key codes (`u32` form).
mod vk {
    pub const BACK: u32 = 0x08;
    pub const TAB: u32 = 0x09;
    pub const RETURN: u32 = 0x0D;
    pub const CAPITAL: u32 = 0x14; // Caps Lock
    pub const ESCAPE: u32 = 0x1B;
    pub const SPACE: u32 = 0x20;

    pub const PRIOR: u32 = 0x21; // Page Up
    pub const NEXT: u32 = 0x22; // Page Down
    pub const END: u32 = 0x23;
    pub const HOME: u32 = 0x24;
    pub const LEFT: u32 = 0x25;
    pub const UP: u32 = 0x26;
    pub const RIGHT: u32 = 0x27;
    pub const DOWN: u32 = 0x28;
    pub const INSERT: u32 = 0x2D;
    pub const DELETE: u32 = 0x2E;

    pub const NUMPAD0: u32 = 0x60;
    pub const NUMPAD1: u32 = 0x61;
    pub const NUMPAD2: u32 = 0x62;
    pub const NUMPAD3: u32 = 0x63;
    pub const NUMPAD4: u32 = 0x64;
    pub const NUMPAD5: u32 = 0x65;
    pub const NUMPAD6: u32 = 0x66;
    pub const NUMPAD7: u32 = 0x67;
    pub const NUMPAD8: u32 = 0x68;
    pub const NUMPAD9: u32 = 0x69;
    pub const MULTIPLY: u32 = 0x6A;
    pub const ADD: u32 = 0x6B;
    pub const SUBTRACT: u32 = 0x6D;
    pub const DECIMAL: u32 = 0x6E;
    pub const DIVIDE: u32 = 0x6F;

    pub const F1: u32 = 0x70;
    pub const F2: u32 = 0x71;
    pub const F3: u32 = 0x72;
    pub const F4: u32 = 0x73;
    pub const F5: u32 = 0x74;
    pub const F6: u32 = 0x75;
    pub const F7: u32 = 0x76;
    pub const F8: u32 = 0x77;
    pub const F9: u32 = 0x78;
    pub const F10: u32 = 0x79;
    pub const F11: u32 = 0x7A;
    pub const F12: u32 = 0x7B;
    pub const F13: u32 = 0x7C;
    pub const F14: u32 = 0x7D;
    pub const F15: u32 = 0x7E;
    pub const F16: u32 = 0x7F;
    pub const F17: u32 = 0x80;
    pub const F18: u32 = 0x81;
    pub const F19: u32 = 0x82;
    pub const F20: u32 = 0x83;

    pub const NUMLOCK: u32 = 0x90;
    pub const SCROLL: u32 = 0x91;

    pub const OEM_1: u32 = 0xBA; // ;:
    pub const OEM_PLUS: u32 = 0xBB; // =+
    pub const OEM_COMMA: u32 = 0xBC; // ,<
    pub const OEM_MINUS: u32 = 0xBD; // -_
    pub const OEM_PERIOD: u32 = 0xBE; // .>
    pub const OEM_2: u32 = 0xBF; // /?
    pub const OEM_3: u32 = 0xC0; // `~
    pub const OEM_4: u32 = 0xDB; // [{
    pub const OEM_5: u32 = 0xDC; // \|
    pub const OEM_6: u32 = 0xDD; // ]}
    pub const OEM_7: u32 = 0xDE; // '"
    pub const OEM_8: u32 = 0xDF;
    pub const OEM_102: u32 = 0xE2; // ISO extra key
}

/// Convert a Windows virtual key code to a `Key`.
///
/// The `is_extended` flag distinguishes keys like numpad Enter from main Enter.
pub(crate) fn vk_to_key(vk_code: u32, is_extended: bool) -> Option<Key> {
    match vk_code {
        // Letters A-Z (0x41-0x5A)
        0x41 => Some(Key::A),
        0x42 => Some(Key::B),
        0x43 => Some(Key::C),
        0x44 => Some(Key::D),
        0x45 => Some(Key::E),
        0x46 => Some(Key::F),
        0x47 => Some(Key::G),
        0x48 => Some(Key::H),
        0x49 => Some(Key::I),
        0x4A => Some(Key::J),
        0x4B => Some(Key::K),
        0x4C => Some(Key::L),
        0x4D => Some(Key::M),
        0x4E => Some(Key::N),
        0x4F => Some(Key::O),
        0x50 => Some(Key::P),
        0x51 => Some(Key::Q),
        0x52 => Some(Key::R),
        0x53 => Some(Key::S),
        0x54 => Some(Key::T),
        0x55 => Some(Key::U),
        0x56 => Some(Key::V),
        0x57 => Some(Key::W),
        0x58 => Some(Key::X),
        0x59 => Some(Key::Y),
        0x5A => Some(Key::Z),

        // Numbers 0-9 (0x30-0x39)
        0x30 => Some(Key::Num0),
        0x31 => Some(Key::Num1),
        0x32 => Some(Key::Num2),
        0x33 => Some(Key::Num3),
        0x34 => Some(Key::Num4),
        0x35 => Some(Key::Num5),
        0x36 => Some(Key::Num6),
        0x37 => Some(Key::Num7),
        0x38 => Some(Key::Num8),
        0x39 => Some(Key::Num9),

        // Numpad keys - these are always distinct from main keys
        vk::NUMPAD0 => Some(Key::Keypad0),
        vk::NUMPAD1 => Some(Key::Keypad1),
        vk::NUMPAD2 => Some(Key::Keypad2),
        vk::NUMPAD3 => Some(Key::Keypad3),
        vk::NUMPAD4 => Some(Key::Keypad4),
        vk::NUMPAD5 => Some(Key::Keypad5),
        vk::NUMPAD6 => Some(Key::Keypad6),
        vk::NUMPAD7 => Some(Key::Keypad7),
        vk::NUMPAD8 => Some(Key::Keypad8),
        vk::NUMPAD9 => Some(Key::Keypad9),
        vk::MULTIPLY => Some(Key::KeypadMultiply),
        vk::ADD => Some(Key::KeypadPlus),
        vk::SUBTRACT => Some(Key::KeypadMinus),
        vk::DECIMAL => Some(Key::KeypadDecimal),
        vk::DIVIDE => Some(Key::KeypadDivide),

        // Return - extended flag means numpad enter
        vk::RETURN if is_extended => Some(Key::KeypadEnter),
        vk::RETURN => Some(Key::Return),

        // Function keys
        vk::F1 => Some(Key::F1),
        vk::F2 => Some(Key::F2),
        vk::F3 => Some(Key::F3),
        vk::F4 => Some(Key::F4),
        vk::F5 => Some(Key::F5),
        vk::F6 => Some(Key::F6),
        vk::F7 => Some(Key::F7),
        vk::F8 => Some(Key::F8),
        vk::F9 => Some(Key::F9),
        vk::F10 => Some(Key::F10),
        vk::F11 => Some(Key::F11),
        vk::F12 => Some(Key::F12),
        vk::F13 => Some(Key::F13),
        vk::F14 => Some(Key::F14),
        vk::F15 => Some(Key::F15),
        vk::F16 => Some(Key::F16),
        vk::F17 => Some(Key::F17),
        vk::F18 => Some(Key::F18),
        vk::F19 => Some(Key::F19),
        vk::F20 => Some(Key::F20),

        // Special keys
        vk::BACK => Some(Key::Delete), // Backspace
        vk::DELETE => Some(Key::ForwardDelete),
        vk::INSERT => Some(Key::Insert),
        vk::TAB => Some(Key::Tab),
        vk::ESCAPE => Some(Key::Escape),
        vk::SPACE => Some(Key::Space),
        vk::PRIOR => Some(Key::PageUp),
        vk::NEXT => Some(Key::PageDown),
        vk::END => Some(Key::End),
        vk::HOME => Some(Key::Home),
        vk::LEFT => Some(Key::LeftArrow),
        vk::UP => Some(Key::UpArrow),
        vk::RIGHT => Some(Key::RightArrow),
        vk::DOWN => Some(Key::DownArrow),

        // Punctuation (OEM keys - US layout)
        vk::OEM_1 => Some(Key::Semicolon),
        vk::OEM_PLUS => Some(Key::Equal),
        vk::OEM_COMMA => Some(Key::Comma),
        vk::OEM_MINUS => Some(Key::Minus),
        vk::OEM_PERIOD => Some(Key::Period),
        vk::OEM_2 => Some(Key::Slash),
        vk::OEM_3 => Some(Key::Grave),
        vk::OEM_4 => Some(Key::LeftBracket),
        vk::OEM_5 => Some(Key::Backslash),
        vk::OEM_6 => Some(Key::RightBracket),
        vk::OEM_7 => Some(Key::Quote),
        vk::OEM_8 => Some(Key::Grave),     // backtick on UK layout
        vk::OEM_102 => Some(Key::Section), // ISO extra key

        // Lock keys
        vk::CAPITAL => Some(Key::CapsLock),
        vk::NUMLOCK => Some(Key::NumLock),
        vk::SCROLL => Some(Key::ScrollLock),

        _ => None,
    }
}

#[cfg(test)]
pub(crate) const ALL: [Key; 114] = [
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::Num0,
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::F16,
    Key::F17,
    Key::F18,
    Key::F19,
    Key::F20,
    Key::Space,
    Key::Return,
    Key::Tab,
    Key::Escape,
    Key::Delete,
    Key::ForwardDelete,
    Key::Insert,
    Key::Home,
    Key::End,
    Key::PageUp,
    Key::PageDown,
    Key::LeftArrow,
    Key::RightArrow,
    Key::UpArrow,
    Key::DownArrow,
    Key::Minus,
    Key::Equal,
    Key::LeftBracket,
    Key::RightBracket,
    Key::Backslash,
    Key::Semicolon,
    Key::Quote,
    Key::Comma,
    Key::Period,
    Key::Slash,
    Key::Grave,
    Key::Section,
    Key::JisYen,
    Key::JisUnderscore,
    Key::JisEisu,
    Key::JisKana,
    Key::Keypad0,
    Key::Keypad1,
    Key::Keypad2,
    Key::Keypad3,
    Key::Keypad4,
    Key::Keypad5,
    Key::Keypad6,
    Key::Keypad7,
    Key::Keypad8,
    Key::Keypad9,
    Key::KeypadDecimal,
    Key::KeypadMultiply,
    Key::KeypadPlus,
    Key::KeypadClear,
    Key::KeypadDivide,
    Key::KeypadEnter,
    Key::KeypadMinus,
    Key::KeypadEquals,
    Key::KeypadComma,
    Key::CapsLock,
    Key::ScrollLock,
    Key::NumLock,
    Key::MouseLeft,
    Key::MouseRight,
    Key::MouseMiddle,
    Key::MouseX1,
    Key::MouseX2,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_letters() {
        assert_eq!("a".parse::<Key>().unwrap(), Key::A);
        assert_eq!("A".parse::<Key>().unwrap(), Key::A);
        assert_eq!("z".parse::<Key>().unwrap(), Key::Z);
    }

    #[test]
    fn parse_numbers() {
        assert_eq!("0".parse::<Key>().unwrap(), Key::Num0);
        assert_eq!("9".parse::<Key>().unwrap(), Key::Num9);
        assert_eq!("num5".parse::<Key>().unwrap(), Key::Num5);
    }

    #[test]
    fn parse_function_keys() {
        assert_eq!("F1".parse::<Key>().unwrap(), Key::F1);
        assert_eq!("f12".parse::<Key>().unwrap(), Key::F12);
        assert_eq!("F20".parse::<Key>().unwrap(), Key::F20);
    }

    #[test]
    fn parse_special_keys() {
        assert_eq!("Space".parse::<Key>().unwrap(), Key::Space);
        assert_eq!("return".parse::<Key>().unwrap(), Key::Return);
        assert_eq!("enter".parse::<Key>().unwrap(), Key::Return);
        assert_eq!("Tab".parse::<Key>().unwrap(), Key::Tab);
        assert_eq!("Escape".parse::<Key>().unwrap(), Key::Escape);
        assert_eq!("esc".parse::<Key>().unwrap(), Key::Escape);
        assert_eq!("Delete".parse::<Key>().unwrap(), Key::Delete);
        assert_eq!("backspace".parse::<Key>().unwrap(), Key::Delete);
    }

    #[test]
    fn parse_arrow_keys() {
        assert_eq!("Left".parse::<Key>().unwrap(), Key::LeftArrow);
        assert_eq!("leftarrow".parse::<Key>().unwrap(), Key::LeftArrow);
        assert_eq!("Right".parse::<Key>().unwrap(), Key::RightArrow);
        assert_eq!("Up".parse::<Key>().unwrap(), Key::UpArrow);
        assert_eq!("Down".parse::<Key>().unwrap(), Key::DownArrow);
    }

    #[test]
    fn parse_punctuation() {
        assert_eq!("-".parse::<Key>().unwrap(), Key::Minus);
        assert_eq!("minus".parse::<Key>().unwrap(), Key::Minus);
        assert_eq!("=".parse::<Key>().unwrap(), Key::Equal);
        assert_eq!("[".parse::<Key>().unwrap(), Key::LeftBracket);
        assert_eq!("]".parse::<Key>().unwrap(), Key::RightBracket);
        assert_eq!("/".parse::<Key>().unwrap(), Key::Slash);
        assert_eq!("`".parse::<Key>().unwrap(), Key::Grave);
    }

    #[test]
    fn parse_jis_and_keypad_keys() {
        assert_eq!("¥".parse::<Key>().unwrap(), Key::JisYen);
        assert_eq!("yen".parse::<Key>().unwrap(), Key::JisYen);
        assert_eq!("jisunderscore".parse::<Key>().unwrap(), Key::JisUnderscore);
        assert_eq!("eisu".parse::<Key>().unwrap(), Key::JisEisu);
        assert_eq!("英数".parse::<Key>().unwrap(), Key::JisEisu);
        assert_eq!("kana".parse::<Key>().unwrap(), Key::JisKana);
        assert_eq!("keypad0".parse::<Key>().unwrap(), Key::Keypad0);
        assert_eq!("keypad.".parse::<Key>().unwrap(), Key::KeypadDecimal);
        assert_eq!("keypad*".parse::<Key>().unwrap(), Key::KeypadMultiply);
        assert_eq!("keypadenter".parse::<Key>().unwrap(), Key::KeypadEnter);
        assert_eq!("keypad=".parse::<Key>().unwrap(), Key::KeypadEquals);
        assert_eq!("keypad,".parse::<Key>().unwrap(), Key::KeypadComma);
    }

    #[test]
    fn parse_lock_and_mouse_keys() {
        assert_eq!("capslock".parse::<Key>().unwrap(), Key::CapsLock);
        assert_eq!("scroll".parse::<Key>().unwrap(), Key::ScrollLock);
        assert_eq!("numlock".parse::<Key>().unwrap(), Key::NumLock);
        assert_eq!("lmb".parse::<Key>().unwrap(), Key::MouseLeft);
        assert_eq!("back".parse::<Key>().unwrap(), Key::MouseX1);
        assert_eq!("forward".parse::<Key>().unwrap(), Key::MouseX2);
    }

    #[test]
    fn parse_unknown_key_fails() {
        assert!("unknown".parse::<Key>().is_err());
        assert!("".parse::<Key>().is_err());
    }

    #[test]
    fn key_display_roundtrip_all_variants() {
        for key in ALL {
            let displayed = format!("{}", key);
            let parsed: Key = displayed.parse().unwrap_or_else(|error| {
                panic!("Roundtrip failed for {key:?} (displayed {displayed:?}): {error}")
            });
            assert_eq!(parsed, key, "Roundtrip failed for {key:?}");
        }
    }

    #[test]
    fn vk_to_key_letters_and_numbers() {
        assert_eq!(vk_to_key(0x41, false), Some(Key::A));
        assert_eq!(vk_to_key(0x30, false), Some(Key::Num0));
        assert_eq!(vk_to_key(0x39, false), Some(Key::Num9));
        assert_eq!(vk_to_key(0x60, false), Some(Key::Keypad0));
        assert_eq!(vk_to_key(0x70, false), Some(Key::F1));
        assert_eq!(vk_to_key(0x25, false), Some(Key::LeftArrow));
        assert_eq!(vk_to_key(0xBA, false), Some(Key::Semicolon));
    }

    #[test]
    fn vk_to_key_return_extended() {
        assert_eq!(vk_to_key(0x0D, false), Some(Key::Return));
        assert_eq!(vk_to_key(0x0D, true), Some(Key::KeypadEnter));
    }

    #[test]
    fn vk_to_key_unmapped_is_none() {
        // Print Screen (0x2C) is not mapped.
        assert_eq!(vk_to_key(0x2C, false), None);
        // 0xA0 is a modifier, not a Key.
        assert_eq!(vk_to_key(0xA0, false), None);
    }
}
