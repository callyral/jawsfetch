use crate::{Color, InfoKey, read_nth_line_from_file};

#[derive (Clone, Copy)]
pub struct KernelOptions {
    pub long: bool
}

pub fn print_kernel(options: KernelOptions, color: Color) {
    let kernel_line = read_nth_line_from_file(0, "/proc/version");

    if options.long {
        println!("{}{}{}{}", color, InfoKey::Kernel, Color::Default, kernel_line);
    } else {
        let kernel_tokens: Vec<&str> = kernel_line.split(' ').collect();
        let kernel_shortened: &str = &kernel_tokens[0..3].join(" ");

        println!("{}{}{}{}", color, InfoKey::Kernel, Color::Default, kernel_shortened);
    }
}
