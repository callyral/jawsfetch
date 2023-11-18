use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::env;

#[macro_export]
macro_rules! module {
    // Use all items in module
    ($mod_name:tt) => {
        mod $mod_name;
        use $mod_name::*;
    };
}

pub fn read_nth_line_from_file(n: usize, file_path: &str) -> String {
    let reader = BufReader::new(File::open(file_path).expect(format!("Failed to open: {file_path}").as_str()));
    reader.lines()
        .nth(n)
        .unwrap_or_else(|| panic!("Line {n} of {file_path} does not exist"))
        .unwrap_or_else(|_| panic!("Could not read line {n} from {file_path}"))
}

pub fn read_file<S: AsRef<str> + std::convert::AsRef<std::path::Path>>(file_path: S) -> Option<String> {
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(string) => Some(string),
        Err(_) => None
    }
}

pub fn read_config_file<S: AsRef<str>>(name: S) -> Option<String> {
    let path =
        match env::var("XDG_CONFIG_HOME").is_ok() {
            // Ok to use unwrap here since it is already known that $XDG_CONFIG_HOME is set
            true => env::var("XDG_CONFIG_HOME").unwrap() + "/jawsfetch/" + name.as_ref(),
            false => env::var("HOME").expect("No such environment variable: $HOME") + "/.config/jawsfetch/" + name.as_ref(),
        };
    read_file(path)
}
