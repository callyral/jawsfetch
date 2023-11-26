use crate::Color;

use clap::Parser;

#[derive (Parser, Debug)]
pub struct Arguments {
    /// Select color
    #[arg(short, long, default_value_t = String::from(Color::Blue))]
    pub color: String,

    /// Disable bold colors
    #[arg(short = 'b', long)]
    pub no_bold: bool,

    /// Long kernel version
    #[arg(short, long)]
    pub kernel_long: bool,

    /// Long shell path
    #[arg(short, long)]
    pub shell_long: bool
}
