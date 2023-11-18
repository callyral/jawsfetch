use crate::{Color, InfoKey};

use std::env;

#[derive (Clone, Copy)]
pub struct ShellOptions {
    pub long: bool
}

pub fn print_shell(options: ShellOptions, color: Color) {
    let shell: String = {
        let local: String =
            if env::var("SHELL").is_ok() {
                env::var("SHELL").unwrap()
            } else {
                "unknown".to_string()
            };

        if options.long {
            local
        } else {
            local.split("/").last().expect("Unable to get last element of shell path").to_string()
        }
    };

    println!("{}{}{}{}", color, InfoKey::Shell, Color::Default, shell);
}
