use crate::read_nth_line_from_file;
use crate::color::Color;
use crate::info_key::InfoKey;

pub fn print_uptime(color: Color) {
    let uptime_file_path: &str = "/proc/uptime";
    let uptime_line = read_nth_line_from_file(0, uptime_file_path);
    let uptime_tokens: Vec<&str> = uptime_line.split(" ").collect();

    let raw_seconds: f32 = uptime_tokens[0].parse().expect("Failed to parse uptime into float");
    // During conversion, out-of-range values are saturated and NaN is zero. I believe this is
    // acceptable for this use case.
    let minutes: usize = (raw_seconds / 60.) as usize;
    let seconds: usize = (raw_seconds % 60.) as usize;

    // Formatting to String
    let uptime_string: String =
        match seconds {
            0 => format!("{}m", minutes),
            _ => {
                match minutes {
                    0 => format!("{}s", seconds),
                    _ => format!("{}m {}s", minutes, seconds),
                }
            }
        };

    println!("{}{}{}{}", color, InfoKey::Uptime.as_str(), Color::Default, uptime_string);
}
