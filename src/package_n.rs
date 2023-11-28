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

fn get_package_number() -> Option<usize> {
    // If at any point there is failure in this function, None is returned.
    let package_command = match determine_package_command() {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Failed to determine package command: {}", e);
            return None;
        }
    };
    let package1: Child = match Command::new(package_command[0])
            .args(&package_command[1..])
            .stdout(Stdio::piped())
            .spawn() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to spawn '{}': {}", package_command.join(" "), e);
                    return None;
                }
            };

    let package1_out: ChildStdout = match package1.stdout {
        Some(o) => o,
        _ => {
            eprintln!("Failed to get stdout from '{}'", package_command.join(" "));
            return None;
        },
    };

    let package2: Child = match Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(package1_out))
            .stdout(Stdio::piped())
            .spawn() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to spawn 'wc -l': {}", e);
                    return None;
                }
            };

    let package_output: Output = match package2.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to wait for output of '... | wc -l': {}", e);
            return None;
        }
    };

    let package_string: &[u8] = package_output.stdout.as_slice();

    let package_string: &str = match str::from_utf8(package_string) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to turn package amount byte array into string: {}", e);
            return None;
        }
    };

    Some(match package_string.trim().parse::<usize>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse package amount into usize: {}", e);
            return None;
        }
    })
}

pub fn print_package_number(color: Color) {
    let package_number: usize = match get_package_number() {
        Some(v) => v,
        _ => return,
    };

    println!("{}{}{}{}", color, InfoKey::Packages, Color::Default, package_number);
}
