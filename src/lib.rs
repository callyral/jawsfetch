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
    let reader = BufReader::new(File::open(file_path).expect("Failed to open: {file_path}"));
    reader.lines()
        .nth(n)
        .expect("Line {n} of {file_path} does not exist")
        .expect("Could not read line {n} from {file_path}")
}

pub fn read_file<S: AsRef<str> + std::convert::AsRef<std::path::Path>>(file_path: S) -> Option<String> {
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(string) => return Some(string),
        Err(_) => return None
    };
}

pub fn read_config_file<S: AsRef<str>>(name: S) -> Option<String> {
    let path =
        match env::var("XDG_CONFIG_HOME").is_ok() {
            true => env::var("XDG_CONFIG_HOME").unwrap() + "/jawsfetch/" + name.as_ref(),
            false => env::var("HOME").expect("No such environment variable: $HOME") + "/.config/jawsfetch/" + name.as_ref(),
        };
    read_file(path)
}
