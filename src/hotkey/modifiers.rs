//! Modifier-key bitflags, string parsing, matching, and Windows virtual-key mapping.
//!
//! Hand-rolled replacement for the `bitflags`-derived `Modifiers` of the
//! `handy-keys` crate v0.2.4 (MIT). The bit layout and semantics are identical.

use std::fmt;
use std::ops::{BitOr, BitOrAssign};
use std::str::FromStr;

use crate::hotkey::Error;

/// Modifier keys for hotkey combinations.
///
/// Individual flags track which side (left/right) was pressed.
/// Compound aliases (`CMD`, `SHIFT`, ...) match either side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers(u32);

impl Modifiers {
    // Individual side-specific flags
    pub const CMD_LEFT: Self = Self(1 << 0);
    pub const SHIFT_LEFT: Self = Self(1 << 1);
    pub const CTRL_LEFT: Self = Self(1 << 2);
    pub const OPT_LEFT: Self = Self(1 << 3);
    pub const FN: Self = Self(1 << 4);
    pub const CMD_RIGHT: Self = Self(1 << 5);
    pub const SHIFT_RIGHT: Self = Self(1 << 6);
    pub const CTRL_RIGHT: Self = Self(1 << 7);
    pub const OPT_RIGHT: Self = Self(1 << 8);

    // Compound aliases — "either side"
    pub const CMD: Self = Self(Self::CMD_LEFT.0 | Self::CMD_RIGHT.0);
    pub const SHIFT: Self = Self(Self::SHIFT_LEFT.0 | Self::SHIFT_RIGHT.0);
    pub const CTRL: Self = Self(Self::CTRL_LEFT.0 | Self::CTRL_RIGHT.0);
    pub const OPT: Self = Self(Self::OPT_LEFT.0 | Self::OPT_RIGHT.0);

    /// All defined bits, used by [`Modifiers::from_bits_truncate`].
    const ALL_BITS: u32 = 0x1FF;

    /// No modifiers set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns `true` if all bits in `other` are set in `self`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns `true` if no modifiers are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The raw bits of this set.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Build a set from raw bits, dropping any bits outside the known flags.
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(bits & Self::ALL_BITS)
    }

    /// Check whether `self` (as a hotkey pattern) matches `event` (the actual
    /// modifier state).
    ///
    /// For each modifier group (Cmd, Shift, Ctrl, Opt):
    /// - Hotkey has both bits (compound): event must have at least one bit from the group
    /// - Hotkey has a specific side: event must have that specific side (extra same-group bits OK)
    /// - Hotkey has neither: event must not have either bit from the group
    ///
    /// FN is matched exactly.
    pub fn matches(self, event: Self) -> bool {
        for &(left, right, _compound) in &GROUPS {
            let hotkey_has_left = self.contains(left);
            let hotkey_has_right = self.contains(right);
            let event_has_left = event.contains(left);
            let event_has_right = event.contains(right);
            let event_has_any = event_has_left || event_has_right;

            if hotkey_has_left && hotkey_has_right {
                // Compound: event must have at least one
                if !event_has_any {
                    return false;
                }
            } else if hotkey_has_left {
                // Specific left: event must have left
                if !event_has_left {
                    return false;
                }
            } else if hotkey_has_right {
                // Specific right: event must have right
                if !event_has_right {
                    return false;
                }
            } else {
                // Hotkey doesn't use this group: event must not have it
                if event_has_any {
                    return false;
                }
            }
        }

        // FN: exact match
        self.contains(Self::FN) == event.contains(Self::FN)
    }

    /// Parse a single modifier name (case-insensitive).
    pub(crate) fn parse_single(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            // Compound (either side)
            "cmd" | "command" | "meta" | "super" | "win" | "windows" => Some(Self::CMD),
            "shift" => Some(Self::SHIFT),
            "ctrl" | "control" => Some(Self::CTRL),
            "opt" | "option" | "alt" => Some(Self::OPT),
            "fn" | "function" => Some(Self::FN),

            // Left-specific
            "cmdleft" | "cmd_left" | "lcmd" | "commandleft" | "command_left" | "lcommand"
            | "superleft" | "super_left" | "winleft" | "win_left" | "windowsleft"
            | "windows_left" | "metaleft" | "meta_left" => Some(Self::CMD_LEFT),
            "shiftleft" | "shift_left" | "lshift" => Some(Self::SHIFT_LEFT),
            "ctrlleft" | "ctrl_left" | "lctrl" | "controlleft" | "control_left" | "lcontrol" => {
                Some(Self::CTRL_LEFT)
            }
            "optleft" | "opt_left" | "lopt" | "optionleft" | "option_left" | "loption"
            | "altleft" | "alt_left" | "lalt" => Some(Self::OPT_LEFT),

            // Right-specific
            "cmdright" | "cmd_right" | "rcmd" | "commandright" | "command_right" | "rcommand"
            | "superright" | "super_right" | "winright" | "win_right" | "windowsright"
            | "windows_right" | "metaright" | "meta_right" => Some(Self::CMD_RIGHT),
            "shiftright" | "shift_right" | "rshift" => Some(Self::SHIFT_RIGHT),
            "ctrlright" | "ctrl_right" | "rctrl" | "controlright" | "control_right"
            | "rcontrol" => Some(Self::CTRL_RIGHT),
            "optright" | "opt_right" | "ropt" | "optionright" | "option_right" | "roption"
            | "altright" | "alt_right" | "ralt" | "altgr" => Some(Self::OPT_RIGHT),

            _ => None,
        }
    }
}

/// All modifier groups as (left, right, compound) triples.
const GROUPS: [(Modifiers, Modifiers, Modifiers); 4] = [
    (Modifiers::CMD_LEFT, Modifiers::CMD_RIGHT, Modifiers::CMD),
    (
        Modifiers::SHIFT_LEFT,
        Modifiers::SHIFT_RIGHT,
        Modifiers::SHIFT,
    ),
    (Modifiers::CTRL_LEFT, Modifiers::CTRL_RIGHT, Modifiers::CTRL),
    (Modifiers::OPT_LEFT, Modifiers::OPT_RIGHT, Modifiers::OPT),
];

impl BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Modifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        // Ctrl group
        if self.contains(Modifiers::CTRL) {
            parts.push("Ctrl");
        } else if self.contains(Modifiers::CTRL_LEFT) {
            parts.push("CtrlLeft");
        } else if self.contains(Modifiers::CTRL_RIGHT) {
            parts.push("CtrlRight");
        }

        // Opt group
        if self.contains(Modifiers::OPT) {
            parts.push("Opt");
        } else if self.contains(Modifiers::OPT_LEFT) {
            parts.push("OptLeft");
        } else if self.contains(Modifiers::OPT_RIGHT) {
            parts.push("OptRight");
        }

        // Shift group
        if self.contains(Modifiers::SHIFT) {
            parts.push("Shift");
        } else if self.contains(Modifiers::SHIFT_LEFT) {
            parts.push("ShiftLeft");
        } else if self.contains(Modifiers::SHIFT_RIGHT) {
            parts.push("ShiftRight");
        }

        // Cmd group
        if self.contains(Modifiers::CMD) {
            parts.push("Cmd");
        } else if self.contains(Modifiers::CMD_LEFT) {
            parts.push("CmdLeft");
        } else if self.contains(Modifiers::CMD_RIGHT) {
            parts.push("CmdRight");
        }

        // Fn
        if self.contains(Modifiers::FN) {
            parts.push("Fn");
        }

        write!(f, "{}", parts.join("+"))
    }
}

impl FromStr for Modifiers {
    type Err = Error;

    /// Parse modifiers from a string like "Cmd+Shift" or "Ctrl+Alt".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Self::empty());
        }

        let mut modifiers = Self::empty();
        for part in s.split('+') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            match Self::parse_single(part) {
                Some(m) => modifiers |= m,
                None => return Err(Error::UnknownModifier(part.to_string())),
            }
        }
        Ok(modifiers)
    }
}

/// Convert a Windows virtual key code to a side-specific [`Modifiers`] flag.
pub(crate) fn vk_to_modifier(vk_code: u32) -> Option<Modifiers> {
    match vk_code {
        0xA0 => Some(Modifiers::SHIFT_LEFT),  // LSHIFT
        0xA1 => Some(Modifiers::SHIFT_RIGHT), // RSHIFT
        0x10 => Some(Modifiers::SHIFT_LEFT),  // SHIFT (generic falls back to left)
        0xA2 => Some(Modifiers::CTRL_LEFT),   // LCONTROL
        0xA3 => Some(Modifiers::CTRL_RIGHT),  // RCONTROL
        0x11 => Some(Modifiers::CTRL_LEFT),   // CONTROL (generic falls back to left)
        0xA4 => Some(Modifiers::OPT_LEFT),    // LMENU
        0xA5 => Some(Modifiers::OPT_RIGHT),   // RMENU
        0x12 => Some(Modifiers::OPT_LEFT),    // MENU (generic falls back to left)
        0x5B => Some(Modifiers::CMD_LEFT),    // LWIN
        0x5C => Some(Modifiers::CMD_RIGHT),   // RWIN
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_modifiers() {
        assert_eq!("Cmd".parse::<Modifiers>().unwrap(), Modifiers::CMD);
        assert_eq!("command".parse::<Modifiers>().unwrap(), Modifiers::CMD);
        assert_eq!("meta".parse::<Modifiers>().unwrap(), Modifiers::CMD);
        assert_eq!("super".parse::<Modifiers>().unwrap(), Modifiers::CMD);
        assert_eq!("win".parse::<Modifiers>().unwrap(), Modifiers::CMD);
        assert_eq!("windows".parse::<Modifiers>().unwrap(), Modifiers::CMD);

        assert_eq!("Shift".parse::<Modifiers>().unwrap(), Modifiers::SHIFT);
        assert_eq!("SHIFT".parse::<Modifiers>().unwrap(), Modifiers::SHIFT);

        assert_eq!("Ctrl".parse::<Modifiers>().unwrap(), Modifiers::CTRL);
        assert_eq!("control".parse::<Modifiers>().unwrap(), Modifiers::CTRL);

        assert_eq!("Opt".parse::<Modifiers>().unwrap(), Modifiers::OPT);
        assert_eq!("option".parse::<Modifiers>().unwrap(), Modifiers::OPT);
        assert_eq!("alt".parse::<Modifiers>().unwrap(), Modifiers::OPT);

        assert_eq!("Fn".parse::<Modifiers>().unwrap(), Modifiers::FN);
        assert_eq!("function".parse::<Modifiers>().unwrap(), Modifiers::FN);
    }

    #[test]
    fn parse_side_specific_modifiers() {
        assert_eq!("CmdLeft".parse::<Modifiers>().unwrap(), Modifiers::CMD_LEFT);
        assert_eq!("LCmd".parse::<Modifiers>().unwrap(), Modifiers::CMD_LEFT);
        assert_eq!(
            "CmdRight".parse::<Modifiers>().unwrap(),
            Modifiers::CMD_RIGHT
        );
        assert_eq!("RCmd".parse::<Modifiers>().unwrap(), Modifiers::CMD_RIGHT);

        assert_eq!(
            "ShiftLeft".parse::<Modifiers>().unwrap(),
            Modifiers::SHIFT_LEFT
        );
        assert_eq!(
            "ShiftRight".parse::<Modifiers>().unwrap(),
            Modifiers::SHIFT_RIGHT
        );

        assert_eq!(
            "CtrlLeft".parse::<Modifiers>().unwrap(),
            Modifiers::CTRL_LEFT
        );
        assert_eq!(
            "CtrlRight".parse::<Modifiers>().unwrap(),
            Modifiers::CTRL_RIGHT
        );

        assert_eq!("OptLeft".parse::<Modifiers>().unwrap(), Modifiers::OPT_LEFT);
        assert_eq!(
            "AltRight".parse::<Modifiers>().unwrap(),
            Modifiers::OPT_RIGHT
        );
        assert_eq!("AltGr".parse::<Modifiers>().unwrap(), Modifiers::OPT_RIGHT);
    }

    #[test]
    fn parse_combined_modifiers() {
        assert_eq!(
            "Cmd+Shift".parse::<Modifiers>().unwrap(),
            Modifiers::CMD | Modifiers::SHIFT
        );
        assert_eq!(
            "Ctrl+Alt+Shift".parse::<Modifiers>().unwrap(),
            Modifiers::CTRL | Modifiers::OPT | Modifiers::SHIFT
        );
    }

    #[test]
    fn parse_empty_modifiers() {
        assert_eq!("".parse::<Modifiers>().unwrap(), Modifiers::empty());
        assert_eq!("  ".parse::<Modifiers>().unwrap(), Modifiers::empty());
    }

    #[test]
    fn parse_unknown_modifier_fails() {
        assert!("Unknown".parse::<Modifiers>().is_err());
        assert!("Cmd+Unknown".parse::<Modifiers>().is_err());
    }

    #[test]
    fn modifiers_display() {
        assert_eq!(format!("{}", Modifiers::CMD), "Cmd");
        assert_eq!(format!("{}", Modifiers::SHIFT), "Shift");
        assert_eq!(
            format!("{}", Modifiers::CMD | Modifiers::SHIFT),
            "Shift+Cmd"
        );
    }

    #[test]
    fn modifiers_display_side_specific() {
        assert_eq!(format!("{}", Modifiers::CMD_LEFT), "CmdLeft");
        assert_eq!(format!("{}", Modifiers::CMD_RIGHT), "CmdRight");
        assert_eq!(format!("{}", Modifiers::SHIFT_LEFT), "ShiftLeft");
        assert_eq!(format!("{}", Modifiers::CTRL_RIGHT), "CtrlRight");
        assert_eq!(format!("{}", Modifiers::OPT_LEFT), "OptLeft");
    }

    #[test]
    fn matches_compound_hotkey() {
        // Compound "Cmd" matches either side
        let hotkey = Modifiers::CMD;
        assert!(hotkey.matches(Modifiers::CMD_LEFT));
        assert!(hotkey.matches(Modifiers::CMD_RIGHT));
        assert!(hotkey.matches(Modifiers::CMD_LEFT | Modifiers::CMD_RIGHT));
        assert!(!hotkey.matches(Modifiers::empty()));
        assert!(!hotkey.matches(Modifiers::SHIFT_LEFT));
    }

    #[test]
    fn matches_side_specific_hotkey() {
        // Specific "CmdLeft" requires left
        let hotkey = Modifiers::CMD_LEFT;
        assert!(hotkey.matches(Modifiers::CMD_LEFT));
        assert!(!hotkey.matches(Modifiers::CMD_RIGHT));
        // Both sides pressed: left is still present, so it matches
        assert!(hotkey.matches(Modifiers::CMD_LEFT | Modifiers::CMD_RIGHT));
        assert!(!hotkey.matches(Modifiers::empty()));
    }

    #[test]
    fn matches_rejects_extra_groups() {
        // Hotkey is just Cmd, event has Cmd+Shift — should fail (extra group)
        let hotkey = Modifiers::CMD;
        assert!(!hotkey.matches(Modifiers::CMD_LEFT | Modifiers::SHIFT_LEFT));

        // Hotkey is CmdLeft+ShiftLeft, event is CmdLeft+ShiftLeft — OK
        let hotkey = Modifiers::CMD_LEFT | Modifiers::SHIFT_LEFT;
        assert!(hotkey.matches(Modifiers::CMD_LEFT | Modifiers::SHIFT_LEFT));
    }

    #[test]
    fn matches_fn_exact() {
        let hotkey = Modifiers::CMD | Modifiers::FN;
        assert!(hotkey.matches(Modifiers::CMD_LEFT | Modifiers::FN));
        assert!(!hotkey.matches(Modifiers::CMD_LEFT)); // missing FN

        let hotkey = Modifiers::CMD;
        assert!(!hotkey.matches(Modifiers::CMD_LEFT | Modifiers::FN)); // extra FN
    }

    #[test]
    fn matches_empty() {
        let hotkey = Modifiers::empty();
        assert!(hotkey.matches(Modifiers::empty()));
        assert!(!hotkey.matches(Modifiers::CMD_LEFT));
    }

    #[test]
    fn compound_equals_both_sides() {
        assert_eq!(Modifiers::CMD, Modifiers::CMD_LEFT | Modifiers::CMD_RIGHT);
        assert_eq!(
            Modifiers::SHIFT,
            Modifiers::SHIFT_LEFT | Modifiers::SHIFT_RIGHT
        );
        assert_eq!(
            Modifiers::CTRL,
            Modifiers::CTRL_LEFT | Modifiers::CTRL_RIGHT
        );
        assert_eq!(Modifiers::OPT, Modifiers::OPT_LEFT | Modifiers::OPT_RIGHT);
    }

    #[test]
    fn from_bits_truncate_drops_unknown_bits() {
        let m = Modifiers::from_bits_truncate(u32::MAX);
        assert_eq!(m.bits(), 0x1FF);
        assert!(m.contains(Modifiers::OPT_RIGHT));
    }

    #[test]
    fn vk_to_modifier_mappings() {
        assert_eq!(vk_to_modifier(0xA0), Some(Modifiers::SHIFT_LEFT));
        assert_eq!(vk_to_modifier(0xA1), Some(Modifiers::SHIFT_RIGHT));
        assert_eq!(vk_to_modifier(0x10), Some(Modifiers::SHIFT_LEFT));
        assert_eq!(vk_to_modifier(0xA2), Some(Modifiers::CTRL_LEFT));
        assert_eq!(vk_to_modifier(0xA3), Some(Modifiers::CTRL_RIGHT));
        assert_eq!(vk_to_modifier(0x11), Some(Modifiers::CTRL_LEFT));
        assert_eq!(vk_to_modifier(0xA4), Some(Modifiers::OPT_LEFT));
        assert_eq!(vk_to_modifier(0xA5), Some(Modifiers::OPT_RIGHT));
        assert_eq!(vk_to_modifier(0x12), Some(Modifiers::OPT_LEFT));
        assert_eq!(vk_to_modifier(0x5B), Some(Modifiers::CMD_LEFT));
        assert_eq!(vk_to_modifier(0x5C), Some(Modifiers::CMD_RIGHT));
        assert_eq!(vk_to_modifier(0x41), None);
    }
}
