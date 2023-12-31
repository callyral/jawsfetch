use crate::Color;
use crate::InfoKey;
use jawsfetch::read_nth_line_from_file;

fn format_seconds(raw_seconds: f32) -> String {
    // During conversion, out-of-range values are saturated and NaN is zero. I believe this is
    // acceptable for this use case.
    let seconds = (raw_seconds % 60.) as usize;
    let minutes = (raw_seconds / 60. % 60.) as usize;
    let hours   = (raw_seconds / 60. / 60.) as usize;

    match hours {
        0 => match minutes {
            0 => format!("{}s", seconds),
            _ => match seconds {
                0 => format!("{}m", minutes),
                _ => format!("{}m {}s", minutes, seconds)
            }
        }
        _ => match minutes {
            0 => format!("{}h {}s", hours, seconds),
            _ => match seconds {
                0 => format!("{}h {}m", hours, minutes),
                _ => format!("{}h {}m {}s", hours, minutes, seconds)
            }
        }
    }
}

pub fn print_uptime(color: Color) {
    let uptime_file_path: &str = "/proc/uptime";
    let uptime_line = match read_nth_line_from_file(0, uptime_file_path) {
        Ok(v) => v,
        _ => return,
    };
    let uptime_tokens: Vec<&str> = uptime_line.split(' ').collect();

    let raw_seconds: f32 = uptime_tokens[0].parse().expect("Failed to parse uptime into float");

    let uptime_string: String = format_seconds(raw_seconds);

    println!("{}{}{}{}", color, InfoKey::Uptime, Color::Default, uptime_string);
}
