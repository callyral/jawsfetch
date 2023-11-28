use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[macro_export]
macro_rules! module {
    // Use all items in module
    ($mod_name:tt) => {
        mod $mod_name;
        use $mod_name::*;
    };
}

pub fn read_nth_line_from_file(n: usize, file_path: &str) -> Result<String, io::Error> {
    let reader = BufReader::new(
        File::open(file_path).unwrap_or_else(|_| panic!("Failed to open file '{file_path}'")),
    );
    // TODO: make this look better
    reader
        .lines()
        .nth(n)
        .unwrap_or_else(|| panic!("Line {n} of {file_path} does not exist"))
}

pub fn read_config_file(name: &str) -> Result<String, io::Error> {
    let path: PathBuf = match env::var("XDG_CONFIG_HOME") {
        Ok(v) => [v, "jawsfetch".into(), name.into()].iter().collect(),
        Err(_) => [
            home::home_dir().expect("Failed to fetch home directory"),
            "/.config/jawsfetch/".into(),
            name.into(),
        ]
        .iter()
        .collect(),
    };
    fs::read_to_string(path)
}
