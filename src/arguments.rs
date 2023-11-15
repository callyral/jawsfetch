use clap::Parser;

#[derive (Parser, Debug)]
pub struct Arguments {
    /// Select color
    #[arg(short, long, default_value_t = String::from("blue"))]
    pub color: String,

    /// Disable bold colors
    #[arg(short, long)]
    pub no_bold: bool,

    /// Long kernel version
    #[arg(short, long)]
    pub kernel_long: bool
}
