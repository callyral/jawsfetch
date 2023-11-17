use crate::{Color, KernelOptions};

use clap::Parser;

#[derive (Clone, Copy)]
pub struct Options {
    pub color: Color,
    pub bold: bool,
    pub kernel_options: KernelOptions
}

#[derive (Parser, Debug)]
pub struct Arguments {
    /// Select color
    #[arg(short, long, default_value_t = String::from(Color::Blue.as_str().unwrap()))]
    pub color: String,

    /// Disable bold colors
    #[arg(short = 'b', long)]
    pub no_bold: bool,

    /// Long kernel version
    #[arg(short, long)]
    pub kernel_long: bool
}
