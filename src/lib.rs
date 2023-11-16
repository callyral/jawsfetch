use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_export]
macro_rules! module {
    // Use all items in module
    ($mod_name:tt) => {
        mod $mod_name;
        use $mod_name::*;
    };
}

pub fn read_nth_line_from_file(n: usize, file_path: &str) -> String {
    let reader = BufReader::new(File::open(file_path).expect("Failed to open file {file_path}"));
    reader.lines()
        .nth(n)
        .expect("Line {n} of {file_path} does not exist")
        .expect("Could not read line {n} from {file_path}")
}
