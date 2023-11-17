use crate::{Color, InfoKey, read_nth_line_from_file};

#[derive (Clone, Copy)]
pub struct KernelOptions {
    pub shorten: bool
}

pub fn print_kernel(options: KernelOptions, color: Color) {
    if !options.shorten {
        println!("{}{}{}{}", color, InfoKey::Kernel.as_str(), Color::Default, read_nth_line_from_file(0, "/proc/version"));
        return;
    }

    let kernel_line = read_nth_line_from_file(0, "/proc/version");
    let kernel_tokens: Vec<&str> = kernel_line.split(" ").collect();

    print!("{}{}{}", color, InfoKey::Kernel.as_str(), Color::Default);
    kernel_tokens[0..3].iter().for_each(|str| print!("{} ", str));
    println!();
}
