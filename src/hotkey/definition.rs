//! A hotkey definition and its string parsing.
//!
//! Ported from the `handy-keys` crate v0.2.4 (MIT), `types/hotkey.rs`.

use std::fmt;
use std::str::FromStr;

use crate::hotkey::{Error, Key, Modifiers};

/// A hotkey definition - either a key with modifiers, or modifiers only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hotkey {
    pub modifiers: Modifiers,
    pub key: Option<Key>,
}

impl Hotkey {
    /// Create a hotkey with modifiers and/or a key.
    ///
    /// At least one of modifiers or key must be provided.
    /// Returns [`Error::EmptyHotkey`] if both are empty/`None`.
    pub fn new(modifiers: Modifiers, key: impl Into<Option<Key>>) -> Result<Self, Error> {
        let key = key.into();
        if modifiers.is_empty() && key.is_none() {
            return Err(Error::EmptyHotkey);
        }
        Ok(Self { modifiers, key })
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.is_empty() {
            match self.key {
                Some(key) => write!(f, "{key}"),
                None => write!(f, "(none)"),
            }
        } else {
            match self.key {
                Some(key) => write!(f, "{}+{}", self.modifiers, key),
                None => write!(f, "{}", self.modifiers),
            }
        }
    }
}

impl FromStr for Hotkey {
    type Err = Error;

    /// Parse a hotkey from a string like `"Cmd+Shift+K"` or `"Ctrl+Space"`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(Error::EmptyHotkey);
        }

        let mut modifiers = Modifiers::empty();
        let mut key: Option<Key> = None;

        for part in s.split('+').map(str::trim) {
            if part.is_empty() {
                continue;
            }

            // Try to parse as a modifier first.
            if let Some(parsed) = Modifiers::parse_single(part) {
                modifiers |= parsed;
            } else {
                // Not a modifier, so it must be a key.
                if key.is_some() {
                    return Err(Error::InvalidFormat(format!(
                        "multiple keys specified: already have a key, found '{part}'"
                    )));
                }
                key = Some(part.parse::<Key>()?);
            }
        }

        Hotkey::new(modifiers, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_modifier_plus_key() {
        let hotkey: Hotkey = "Cmd+K".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CMD);
        assert_eq!(hotkey.key, Some(Key::K));
    }

    #[test]
    fn parse_keypad_with_spaces() {
        let hotkey: Hotkey = "Ctrl+Keypad0".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CTRL);
        assert_eq!(hotkey.key, Some(Key::Keypad0));

        let hotkey: Hotkey = "Ctrl + Keypad0".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CTRL);
        assert_eq!(hotkey.key, Some(Key::Keypad0));
    }

    #[test]
    fn parse_multiple_modifiers_plus_key() {
        let hotkey: Hotkey = "cmd+shift+k".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CMD | Modifiers::SHIFT);
        assert_eq!(hotkey.key, Some(Key::K));

        let hotkey: Hotkey = "Ctrl+Alt+Delete".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CTRL | Modifiers::OPT);
        assert_eq!(hotkey.key, Some(Key::Delete));
    }

    #[test]
    fn parse_key_only() {
        let hotkey: Hotkey = "F1".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::empty());
        assert_eq!(hotkey.key, Some(Key::F1));

        let hotkey: Hotkey = "Space".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::empty());
        assert_eq!(hotkey.key, Some(Key::Space));
    }

    #[test]
    fn parse_modifiers_only() {
        let hotkey: Hotkey = "Cmd+Shift".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CMD | Modifiers::SHIFT);
        assert_eq!(hotkey.key, None);
    }

    #[test]
    fn parse_side_specific_hotkey() {
        let hotkey: Hotkey = "CtrlRight+Space".parse().unwrap();
        assert_eq!(hotkey.modifiers, Modifiers::CTRL_RIGHT);
        assert_eq!(hotkey.key, Some(Key::Space));

        let hotkey: Hotkey = "CmdLeft+ShiftRight+K".parse().unwrap();
        assert_eq!(
            hotkey.modifiers,
            Modifiers::CMD_LEFT | Modifiers::SHIFT_RIGHT
        );
        assert_eq!(hotkey.key, Some(Key::K));
    }

    #[test]
    fn parse_empty_fails() {
        assert!("".parse::<Hotkey>().is_err());
        assert!("   ".parse::<Hotkey>().is_err());
    }

    #[test]
    fn parse_multiple_keys_fails() {
        assert!("A+B".parse::<Hotkey>().is_err());
        assert!("Cmd+A+B".parse::<Hotkey>().is_err());
    }

    #[test]
    fn parse_case_insensitive() {
        let h1: Hotkey = "CMD+SHIFT+K".parse().unwrap();
        let h2: Hotkey = "cmd+shift+k".parse().unwrap();
        let h3: Hotkey = "Cmd+Shift+K".parse().unwrap();
        assert_eq!(h1, h2);
        assert_eq!(h2, h3);
    }

    #[test]
    fn hotkey_display() {
        let hotkey = Hotkey::new(Modifiers::CMD | Modifiers::SHIFT, Key::K).unwrap();
        assert_eq!(format!("{hotkey}"), "Shift+Cmd+K");

        let key_only = Hotkey::new(Modifiers::empty(), Key::F1).unwrap();
        assert_eq!(format!("{key_only}"), "F1");

        let modifiers_only = Hotkey::new(Modifiers::CTRL, None).unwrap();
        assert_eq!(format!("{modifiers_only}"), "Ctrl");
    }

    #[test]
    fn hotkey_display_roundtrip_keypad() {
        let keypad_keys = [
            Key::KeypadPlus,
            Key::KeypadMinus,
            Key::KeypadMultiply,
            Key::KeypadDivide,
            Key::KeypadDecimal,
            Key::KeypadEquals,
            Key::KeypadEnter,
            Key::KeypadClear,
        ];
        for key in keypad_keys {
            let hotkey = Hotkey::new(Modifiers::empty(), key).unwrap();
            let parsed: Hotkey = format!("{hotkey}").parse().unwrap();
            assert_eq!(parsed, hotkey, "Key-only roundtrip failed for {key:?}");

            let hotkey = Hotkey::new(Modifiers::CMD, key).unwrap();
            let parsed: Hotkey = format!("{hotkey}").parse().unwrap();
            assert_eq!(parsed, hotkey, "Cmd+{key:?} roundtrip failed");
        }
    }

    #[test]
    fn hotkey_new_validates() {
        assert!(Hotkey::new(Modifiers::CMD, Key::K).is_ok());
        assert!(Hotkey::new(Modifiers::CMD | Modifiers::SHIFT, None).is_ok());
        assert!(Hotkey::new(Modifiers::empty(), Key::F1).is_ok());
        assert!(Hotkey::new(Modifiers::empty(), None).is_err());
    }
}
