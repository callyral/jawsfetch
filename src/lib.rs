use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::env;

use home::home_dir;

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
        match env::var("XDG_CONFIG_HOME") {
            // TODO: use a better method to append to path
            Ok(v) => v + "/jawsfetch/" + name.as_ref(),
            Err(_) => home_dir()?.into_os_string().into_string().expect("Failed to fetch home directory") + "/.config/jawsfetch/" + name.as_ref(),
        };
    read_file(path)
}
