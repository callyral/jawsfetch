#[derive (Clone, Copy)]
pub enum InfoKey {
    Invalid,
    Ascii,
    Distro,
    Kernel,
    Session,
    Shell,
    Packages,
    Uptime
}

impl InfoKey {
    pub fn as_str(&self) -> &str {
        match self {
            InfoKey::Distro   => "dis: ",
            InfoKey::Kernel   => "krn: ",
            InfoKey::Session  => "xdg: ",
            InfoKey::Shell    => "shl: ",
            InfoKey::Packages => "pkg: ",
            InfoKey::Uptime   => "upt: ",
            _ => "invalid",
        }
    }

    pub fn as_order_str(&self) -> &str {
        match self {
            InfoKey::Ascii    => "ascii",
            InfoKey::Distro   => "distro",
            InfoKey::Kernel   => "kernel",
            InfoKey::Session  => "session",
            InfoKey::Shell    => "shell",
            InfoKey::Packages => "packages",
            InfoKey::Uptime   => "uptime",
            _ => "invalid"
        }
    }

    pub fn from_order_str(&self, string: &str) -> InfoKey {
        match string {
           "ascii"    => InfoKey::Ascii,
           "distro"   => InfoKey::Distro,
           "kernel"   => InfoKey::Kernel,
           "session"  => InfoKey::Session,
           "shell"    => InfoKey::Shell,
           "packages" => InfoKey::Packages,
           "uptime"   => InfoKey::Uptime,
           _          => InfoKey::Invalid
        }
    }
}
