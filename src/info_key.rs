use std::fmt;

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

impl InfoKey {
    pub fn from_order_key<S: AsRef<str>>(string: S) -> Option<InfoKey> {
        match string.as_ref() {
           "ascii"    => Some(InfoKey::Ascii),
           "distro"   => Some(InfoKey::Distro),
           "kernel"   => Some(InfoKey::Kernel),
           "session"  => Some(InfoKey::Session),
           "shell"    => Some(InfoKey::Shell),
           "packages" => Some(InfoKey::Packages),
           "uptime"   => Some(InfoKey::Uptime),
           _          => None
        }
    }
}

impl fmt::Display for InfoKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoKey::Ascii    => write!(f, ""),
            InfoKey::Distro   => write!(f, "dis: "),
            InfoKey::Kernel   => write!(f, "krn: "),
            InfoKey::Session  => write!(f, "xdg: "),
            InfoKey::Shell    => write!(f, "shl: "),
            InfoKey::Packages => write!(f, "pkg: "),
            InfoKey::Uptime   => write!(f, "upt: ")
        }
    }
}
