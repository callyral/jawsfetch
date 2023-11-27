use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct ParseColorError;

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
    BoldReset
}

impl Color {
    pub fn bold(self) -> Color {
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
            _ => self
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

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Case insensitive
        match s.to_lowercase().as_str() {
            "black"   => Ok(Color::Black),
            "red"     => Ok(Color::Red),
            "green"   => Ok(Color::Green),
            "yellow"  => Ok(Color::Yellow),
            "blue"    => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan"    => Ok(Color::Cyan),
            "white"   => Ok(Color::White),
            "default" => Ok(Color::Default),
            "reset"   => Ok(Color::Reset),
            "bold_black"   => Ok(Color::BoldBlack),
            "bold_red"     => Ok(Color::BoldRed),
            "bold_green"   => Ok(Color::BoldGreen),
            "bold_yellow"  => Ok(Color::BoldYellow),
            "bold_blue"    => Ok(Color::BoldBlue),
            "bold_magenta" => Ok(Color::BoldMagenta),
            "bold_cyan"    => Ok(Color::BoldCyan),
            "bold_white"   => Ok(Color::BoldWhite),
            "bold_default" => Ok(Color::BoldDefault),
            "bold_reset"   => Ok(Color::BoldReset),
            _ => Err(ParseColorError)
        }
    }
}

impl From<Color> for String {
    fn from(col: Color) -> Self {
        match col {
            Color::Black   => "black",
            Color::Red     => "red",
            Color::Green   => "green",
            Color::Yellow  => "yellow",
            Color::Blue    => "blue",
            Color::Magenta => "magenta",
            Color::Cyan    => "cyan",
            Color::White   => "white",
            Color::Default => "default",
            Color::Reset   => "reset",
            Color::BoldBlack   => "bold_black",
            Color::BoldRed     => "bold_red",
            Color::BoldGreen   => "bold_green",
            Color::BoldYellow  => "bold_yellow",
            Color::BoldBlue    => "bold_blue",
            Color::BoldMagenta => "bold_magenta",
            Color::BoldCyan    => "bold_cyan",
            Color::BoldWhite   => "bold_white",
            Color::BoldDefault => "bold_default",
            Color::BoldReset   => "bold_reset"
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_bold() {
        let magenta = Color::Magenta;
        let bold_cyan = Color::BoldCyan;

        // normal -/> normal
        assert_ne!(magenta.bold(), Color::Magenta);

        // make sure .bold() doesn't mutate
        assert_eq!(magenta, Color::Magenta);

        // normal -> bold
        assert_eq!(magenta.bold(), Color::BoldMagenta);

        // bold -> bold
        assert_eq!(bold_cyan.bold(), Color::BoldCyan);
    }

    #[test]
    fn color_from_string() {
        assert_eq!(Color::from_str("blue"), Ok(Color::Blue));
    }

    #[test]
    fn string_from_color() {
        assert_eq!(String::from(Color::Blue), String::from("blue"));
    }
}
