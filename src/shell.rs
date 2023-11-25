use crate::{Color, InfoKey};

use std::env;

#[derive (Clone, Copy)]
pub struct ShellOptions {
    pub long: bool
}

pub fn print_shell(options: ShellOptions, color: Color) {
    let shell: String = {
        let local: String =
            if let Ok(sh) = env::var("SHELL") {
                sh
            } else {
                panic!("SHELL is not defined");
            };

        if options.long {
            local
        } else {
            local.split('/').last().expect("Unable to shorten shell path").to_string()
        }
    };

    println!("{}{}{}{}", color, InfoKey::Shell, Color::Default, shell);
}
