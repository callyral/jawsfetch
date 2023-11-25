use std::fmt;
use phf::phf_map;

#[derive (Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Reset,
    BoldBlack,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldMagenta,
    BoldCyan,
    BoldWhite,
    BoldDefault,
    BoldReset,
}

impl Color {
    const TRANSLATION: phf::Map<&'static str, Color> = phf_map! {
        "black"   => Color::Black,
        "red"     => Color::Red,
        "green"   => Color::Green,
        "yellow"  => Color::Yellow,
        "blue"    => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan"    => Color::Cyan,
        "white"   => Color::White,
        "default" => Color::Default,
        "reset"   => Color::Reset,
        "bold_black"   => Color::BoldBlack,
        "bold_red"     => Color::BoldRed,
        "bold_green"   => Color::BoldGreen,
        "bold_yellow"  => Color::BoldYellow,
        "bold_blue"    => Color::BoldBlue,
        "bold_magenta" => Color::BoldMagenta,
        "bold_cyan"    => Color::BoldCyan,
        "bold_white"   => Color::BoldWhite,
        "bold_default" => Color::BoldDefault,
        "bold_reset"   => Color::BoldReset,
    };

    pub fn as_bold(&self) -> Color {
        match self {
            Color::Black   => Color::BoldBlack,
            Color::Red     => Color::BoldRed,
            Color::Green   => Color::BoldGreen,
            Color::Yellow  => Color::BoldYellow,
            Color::Blue    => Color::BoldBlue,
            Color::Magenta => Color::BoldMagenta,
            Color::Cyan    => Color::BoldCyan,
            Color::White   => Color::BoldWhite,
            Color::Default => Color::BoldDefault,
            Color::Reset   => Color::BoldReset,
            _ => *self
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        let t = Self::TRANSLATION;
        for key in t.keys() {
            let value = t.get(key);
            // Unwrap used because `key` is known to be a valid dictionary key
            if value.unwrap() == self {
                return Some(key);
            }
        }
        None
    }

    pub fn from_string<S: AsRef<str>>(string: S) -> Color {
        let t = Self::TRANSLATION;
        match t.get(string.as_ref()) {
            Some(&color) => color,
            None => Color::Default
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black   => write!(f, "\x1b[0;30m"),
            Color::Red     => write!(f, "\x1b[0;31m"),
            Color::Green   => write!(f, "\x1b[0;32m"),
            Color::Yellow  => write!(f, "\x1b[0;33m"),
            Color::Blue    => write!(f, "\x1b[0;34m"),
            Color::Magenta => write!(f, "\x1b[0;35m"),
            Color::Cyan    => write!(f, "\x1b[0;36m"),
            Color::White   => write!(f, "\x1b[0;37m"),
            Color::Default => write!(f, "\x1b[0;39m"),
            Color::Reset   => write!(f, "\x1b[0;0m"),
            Color::BoldBlack   => write!(f, "\x1b[1;30m"),
            Color::BoldRed     => write!(f, "\x1b[1;31m"),
            Color::BoldGreen   => write!(f, "\x1b[1;32m"),
            Color::BoldYellow  => write!(f, "\x1b[1;33m"),
            Color::BoldBlue    => write!(f, "\x1b[1;34m"),
            Color::BoldMagenta => write!(f, "\x1b[1;35m"),
            Color::BoldCyan    => write!(f, "\x1b[1;36m"),
            Color::BoldWhite   => write!(f, "\x1b[1;37m"),
            Color::BoldDefault => write!(f, "\x1b[1;39m"),
            Color::BoldReset   => write!(f, "\x1b[1;0m"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_bold() {
        assert_eq!(Color::Magenta.as_bold(), Color::BoldMagenta);
        assert_eq!(Color::BoldCyan.as_bold(), Color::BoldCyan);
    }

    #[test]
    fn as_str() {
        assert_eq!(Color::Blue.as_str(), Some("blue"));
    }
}
