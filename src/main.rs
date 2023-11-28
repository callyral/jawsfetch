use jawsfetch::module;
use jawsfetch::read_config_file;

module!(arguments);
module!(color);
module!(info_key);
module!(kernel);
module!(package_n);
module!(shell);
module!(uptime);

use crate::{ascii::*, distro::*, session::*};

use clap::Parser;
use std::str::FromStr;

fn main() {
    let args = Arguments::parse();

    let info_order: Vec<InfoKey> = get_info_order();

    info_order.iter().for_each(|k| print_info(*k, &args));
}

fn get_info_order() -> Vec<InfoKey> {
    let order_file_contents = read_config_file("order");
    if let Some(contents) = order_file_contents {
        let custom: Vec<InfoKey> = contents
            .lines()
            .map(|line| {
                 InfoKey::from_str(line).unwrap_or_else(|_| panic!("Invalid info key: {line}"))
            })
            .collect();
        return custom;
    }

    vec![
        InfoKey::Ascii,
        InfoKey::Distro,
        InfoKey::Kernel,
        InfoKey::Session,
        InfoKey::Shell,
        InfoKey::Packages,
        InfoKey::Uptime
    ]
}

fn print_info(key: InfoKey, args: &Arguments) {
    let color = if args.no_bold {
        Color::from_str(&args.color).unwrap_or(Color::Default)
    } else {
        Color::from_str(&args.color).unwrap_or(Color::Default).bold()
    };

    match key {
        InfoKey::Ascii    => print_ascii(color),
        InfoKey::Distro   => print_distro(color),
        InfoKey::Kernel   => print_kernel(args.kernel_long, color),
        InfoKey::Session  => print_session(color),
        InfoKey::Shell    => print_shell(args.shell_long, color),
        InfoKey::Uptime   => print_uptime(color),
        InfoKey::Packages => print_package_number(color),
    };
}

mod ascii {
    use crate::Color;
    use jawsfetch::read_config_file;

    pub fn print_ascii(color: Color) {
        let ascii: String = read_config_file("ascii")
            .unwrap_or(r"      .
    \_____)\_____
    /--v____ __`<
            )/
            '".to_string() + "\n");
        print!(r"{}{}{}", color, ascii, Color::Default);
    }
}

mod distro {
    use crate::Color;
    use crate::InfoKey;
    use jawsfetch::read_nth_line_from_file;

    pub fn print_distro(color: Color) {
        let distro_file_path: &str = "/etc/os-release";
        let distro_line = read_nth_line_from_file(0, distro_file_path);
        let distro_tokens: Vec<&str> = distro_line.split('=').collect();

        let distro_name: &str = distro_tokens[1]
            .strip_prefix('\"')
            .expect("Unable to strip quote from distro name")
            .strip_suffix('\"')
            .expect("Unable to strip end-quote from distro name");

        println!("{}{}{}{}", color, InfoKey::Distro, Color::Default, distro_name);
    }
}

mod session {
    use crate::Color;
    use crate::InfoKey;
    use std::env;

    pub fn print_session(color: Color) {
        let session: String =
            if env::var("XDG_CURRENT_DESKTOP").is_ok() {
                env::var("XDG_CURRENT_DESKTOP")
            } else {
                env::var("XDG_SESSION_DESKTOP")
            }.unwrap_or_else(|_| "unknown".to_string());
        println!("{}{}{}{}", color, InfoKey::Session, Color::Default, session);
    }
}
