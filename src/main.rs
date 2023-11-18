use jawsfetch::read_nth_line_from_file;
use jawsfetch::read_config_file;
use jawsfetch::module;

module!(color);
module!(info_key);
module!(arguments);

module!(kernel);
module!(package_n);
module!(uptime);

use std::env;
use clap::Parser;

fn main() {
    let args = Arguments::parse();
    let options = Options {
        color: Color::from_string(args.color.as_str()),
        bold: !args.no_bold,
        kernel_options: KernelOptions {
            long: args.kernel_long
        }
    };

    let info_order: Vec<InfoKey> = get_info_order();

    info_order.iter().for_each(|k| print_info(*k, options));
}

fn get_info_order() -> Vec<InfoKey> {
    let order_file_contents = read_config_file("order");
    match order_file_contents {
        Some(contents) => {
            let custom: Vec<InfoKey> = contents.lines().map(|line| InfoKey::from_order_key(line).expect(format!("Invalid info key: {line}").as_str())).collect();
            return custom;
        },
        None => ()
    }

    return vec![InfoKey::Ascii,
                InfoKey::Distro,
                InfoKey::Kernel,
                InfoKey::Session,
                InfoKey::Shell,
                InfoKey::Packages,
                InfoKey::Uptime]
}

fn print_info(key: InfoKey, options: Options) {
    let color =
        match options.bold {
            true => options.color.as_bold(),
            false => options.color
        };
    match key {
        InfoKey::Ascii => print_ascii(color),
        InfoKey::Distro => print_distro(color),
        InfoKey::Kernel => print_kernel(options.kernel_options, color),
        InfoKey::Session => print_session(color),
        InfoKey::Shell => print_shell(color),
        InfoKey::Uptime => print_uptime(color),
        InfoKey::Packages => print_package_number(color),
    };
}

fn print_ascii(color: Color) {
    let ascii: String = read_config_file("ascii")
        .unwrap_or(r"      .
\_____)\_____
/--v____ __`<
        )/
        '".to_string() + "\n");
    print!(r"{}{}{}", color, ascii, Color::Default);
}

fn print_distro(color: Color) {
    let distro_file_path: &str = "/etc/os-release";
    let distro_line = read_nth_line_from_file(0, distro_file_path);
    let distro_tokens: Vec<&str> = distro_line.split("=").collect();

    let distro_name: &str = distro_tokens[1]
        .strip_prefix("\"")
        .expect("Unable to strip quote from distro name")
        .strip_suffix("\"")
        .expect("Unable to strip end-quote from distro name");

    println!("{}{}{}{}", color, InfoKey::Distro, Color::Default, distro_name);
}

fn print_session(color: Color) {
    let session: String =
        if env::var("XDG_CURRENT_DESKTOP").is_ok() {
            env::var("XDG_CURRENT_DESKTOP")
        } else {
            env::var("XDG_SESSION_DESKTOP")
        }.unwrap_or_else(|_| "unknown".to_string());
    println!("{}{}{}{}", color, InfoKey::Session, Color::Default, session);
}

fn print_shell(color: Color) {
    let shell: String =
        if env::var("SHELL").is_ok() {
            env::var("SHELL").unwrap()
        } else {
            "unknown".to_string()
        };
    println!("{}{}{}{}", color, InfoKey::Shell, Color::Default, shell);
}
