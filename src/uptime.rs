use std::fs;

const ERROR_UNABLE_TO_READ_FILE: &str = "couldn't read uptime file";
const ERROR_UPTIME_NOT_FLOAT: &str = "uptime value is not decimal";

fn get_uptime() -> u64 {
    let uptime_file_content = fs::read_to_string("/proc/uptime").expect(ERROR_UNABLE_TO_READ_FILE);

    uptime_file_content
        .split_whitespace()
        .collect::<Vec<&str>>()[0]
        .parse::<f64>()
        .expect(ERROR_UPTIME_NOT_FLOAT) as u64
}

pub fn get_duration_message() -> String {
    let seconds = get_uptime();

    [
        (seconds / 60 / 60 / 24, "day"),
        (seconds / 60 / 60 % 24, "hour"),
        (seconds / 60 % 60, "minute"),
        (seconds % 60, "second"),
    ]
    .iter()
    .filter(|(x, _)| *x > 0)
    .map(|(x, y)| {
        let unit = if (*x) <= 1 {
            (*y).to_string()
        } else {
            format!("{}s", y)
        };

        (*x, unit)
    })
    .map(|(x, y)| format!("{} {}", x, y))
    .collect::<Vec<String>>()
    .join(" ")
}
