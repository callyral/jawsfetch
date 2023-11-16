use crate::color::Color;
use crate::info_key::InfoKey;

use std::process::{Command, Stdio};
use std::str;

fn determine_package_command() -> Option<Vec<&'static str>> {
    let available: Vec<Vec<&str>> = vec![vec!["dpkg", "-l"], vec!["pacman", "-Q"], vec!["xbps-query", "-l"]];
    let mut command: Option<Vec<&str>> = None;
    for cmd in available.iter() {
        let spawned =
            Command::new(cmd[0])
                .args(&cmd[1..])
                .stdout(Stdio::piped())
                .spawn();
        match spawned {
            Ok(_) => command = Some(cmd.to_vec()),
            Err(_) => command = None
        }
    };
    command
}

fn get_package_number() -> usize {
    let package_command = determine_package_command().expect("Unable to determine package command");
    let package_spawned1 =
        Command::new(package_command[0])
            .args(&package_command[1..])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start package command process");
    let package_spawned1_out = package_spawned1.stdout.expect("Failed to open package command stdout");
    let package_spawned2 =
        Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(package_spawned1_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start wc -l process");
    let package_output = package_spawned2.wait_with_output().expect("Failed to wait for wc -l output");
    let package_u8s = package_output.stdout.as_slice();
    let package_string = str::from_utf8(package_u8s).expect("Failed to turn byte array into string");

    package_string.trim().parse::<usize>().expect("Unable to parse package number")
}

pub fn print_package_number(color: Color) {
    let package_number: usize = get_package_number();

    println!("{}{}{}{}", color, InfoKey::Packages.as_str(), Color::Default, package_number);
}
