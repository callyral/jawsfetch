use crate::{Color, InfoKey, read_nth_line_from_file};

pub fn print_kernel(long: bool, color: Color) {
    let kernel_line = read_nth_line_from_file(0, "/proc/version");

    if long {
        println!("{}{}{}{}", color, InfoKey::Kernel, Color::Default, kernel_line);
    } else {
        let kernel_tokens: Vec<&str> = kernel_line.split(' ').collect();
        let kernel_line: &str = &kernel_tokens[0..3].join(" ");

        println!("{}{}{}{}", color, InfoKey::Kernel, Color::Default, kernel_line);
    }
}
