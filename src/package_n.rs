use crate::color::Color;
use crate::info_key::InfoKey;

use std::process::{Command, Output, Stdio};

pub fn print_package_number(color: Color) {
    let package_number: usize = todo!();

    println!("{}{}{}{}", color, InfoKey::Packages.as_str(), Color::Default, package_number);
}
