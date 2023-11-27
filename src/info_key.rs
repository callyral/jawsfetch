use std::fmt;
use std::str::FromStr;

// MAYBE: rename this? not sure if properly named
pub struct ParseInfoKeyError;

#[derive (Clone, Copy)]
pub enum InfoKey {
    Ascii,
    Distro,
    Kernel,
    Session,
    Shell,
    Packages,
    Uptime
}

impl fmt::Display for InfoKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoKey::Ascii    => write!(f, ""), // unused
            InfoKey::Distro   => write!(f, "dis: "),
            InfoKey::Kernel   => write!(f, "krn: "),
            InfoKey::Session  => write!(f, "xdg: "),
            InfoKey::Shell    => write!(f, "shl: "),
            InfoKey::Packages => write!(f, "pkg: "),
            InfoKey::Uptime   => write!(f, "upt: ")
        }
    }
}

impl FromStr for InfoKey {
    type Err = ParseInfoKeyError;
    fn from_str(k: &str) -> Result<Self, Self::Err> {
        match k {
           "ascii"    => Ok(InfoKey::Ascii),
           "distro"   => Ok(InfoKey::Distro),
           "kernel"   => Ok(InfoKey::Kernel),
           "session"  => Ok(InfoKey::Session),
           "shell"    => Ok(InfoKey::Shell),
           "packages" => Ok(InfoKey::Packages),
           "uptime"   => Ok(InfoKey::Uptime),
           _          => Err(ParseInfoKeyError)
        }
    }
}
