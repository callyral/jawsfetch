use crate::Color;
use crate::InfoKey;

use std::process::Child;
use std::process::ChildStdout;
use std::process::Output;
use std::process::{Command, Stdio};
use std::str;
use std::io;

fn determine_package_command() -> Result<Vec<&'static str>, io::Error> {
    let available: Vec<Vec<&str>> = vec![vec!["dpkg", "-l"], vec!["pacman", "-Q"], vec!["xbps-query", "-l"]];

    let mut command: Result<Vec<&str>, io::Error> = Err(io::ErrorKind::NotFound.into());

    available.iter().for_each(|cmd| {
        match Command::new(cmd[0])
                .args(&cmd[1..])
                .stdout(Stdio::piped())
                .spawn() {
                    Ok(_) => command = Ok(cmd.to_vec()),
                    Err(e) => command = Err(e),
                };
    });

    command
}

fn get_package_number() -> Result<usize, io::Error> {
    let package_command = determine_package_command()?;
    let package1: Child = Command::new(package_command[0])
            .args(&package_command[1..])
            .stdout(Stdio::piped())
            .spawn()?;

    let package1_out: ChildStdout = package1
        .stdout
        .unwrap_or_else(|| panic!("Failed to get stdout from '{}'", package_command.join(" ")));

    let package2: Child = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(package1_out))
            .stdout(Stdio::piped())
            .spawn()?;

    let package_output: Output = package2.wait_with_output()?;

    let package_string: &[u8] = package_output.stdout.as_slice();

    let package_string: &str = str::from_utf8(package_string)
        .unwrap_or_else(|e| panic!("Failed to turn package amount byte array into string: {}", e));

    Ok(package_string
       .trim()
       .parse::<usize>()
       .unwrap_or_else(|e| panic!("Failed to parse package amount into usize: {}", e)))
}

pub fn print_package_number(color: Color) {
    let package_number: usize = match get_package_number() {
        Ok(v) => v,
        _ => return,
    };

    println!("{}{}{}{}", color, InfoKey::Packages, Color::Default, package_number);
}
