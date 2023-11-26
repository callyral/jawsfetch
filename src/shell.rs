use crate::{Color, InfoKey};

use std::env;

pub fn print_shell(long: bool, color: Color) {
    let shell: String = {
        let local: String =
            if let Ok(sh) = env::var("SHELL") {
                sh
            } else {
                panic!("SHELL is not defined");
            };

        if long {
            local
        } else {
            // TODO: make this use std::path::Path
            // something like Path::new(local).file_stem()
            local.split('/').last().expect("Unable to shorten shell path").to_string()
        }
    };

    println!("{}{}{}{}", color, InfoKey::Shell, Color::Default, shell);
}
