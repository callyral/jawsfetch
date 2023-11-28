use crate::Color;
use crate::InfoKey;
use jawsfetch::read_nth_line_from_file;

pub fn print_kernel(long: bool, color: Color) {
    let kernel_file_path = "/proc/version";
    let kernel_line: String = match read_nth_line_from_file(0, kernel_file_path) {
        Ok(v) => v,
        _ => return,
    };

    if long {
        println!("{}{}{}{}", color, InfoKey::Kernel, Color::Default, kernel_line);
    } else {
        let kernel_tokens: Vec<&str> = kernel_line.split(' ').collect();
        let kernel_line: &str = &kernel_tokens[0..3].join(" ");

        println!("{}{}{}{}", color, InfoKey::Kernel, Color::Default, kernel_line);
    }
}
