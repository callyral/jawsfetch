use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_nth_line_from_file(n: usize, file_path: &str) -> String {
    let reader = BufReader::new(File::open(file_path).expect("Failed to open file {file_path}"));
    reader.lines()
        .nth(n)
        .expect("Line {n} of {file_path} does not exist")
        .expect("Could not read line {n} from {file_path}")
}
