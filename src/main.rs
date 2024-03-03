use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

const ERROR_CONFIG_FILE_UNREADABLE: &str = "couldn't read config file";
const ERROR_HOME_VARIABLE_NOT_SET: &str = "couldn't find home env variable";
const ERROR_REQUEST_FAILED: &str = "request to Telegram failed";
const ERROR_UNABLE_TO_READ_FILE: &str = "couldn't read uptime file";
const ERROR_UPTIME_NOT_FLOAT: &str = "uptime value is not decimal";

fn get_telegram_endpoint(token: String) -> String {
    format!("https://api.telegram.org/bot{}/sendMessage", token)
}

fn read_config() -> HashMap<String, String> {
    let home = env::var("HOME").expect(ERROR_HOME_VARIABLE_NOT_SET);
    let file = Path::new(&home).join(".config").join("house-monitor.conf");
    let config_file = fs::read_to_string(file).expect(ERROR_CONFIG_FILE_UNREADABLE);

    let mut config = HashMap::new();

    config_file.lines().for_each(|line| {
        let parts = line
            .split('=')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        config.insert(parts[0].to_string(), parts[1].to_string());
    });

    config
}

fn get_uptime() -> u64 {
    let uptime_file_content = fs::read_to_string("/proc/uptime").expect(ERROR_UNABLE_TO_READ_FILE);

    uptime_file_content
        .split_whitespace()
        .collect::<Vec<&str>>()[0]
        .parse::<f64>()
        .expect(ERROR_UPTIME_NOT_FLOAT) as u64
}

fn get_duration_message(seconds: u64) -> String {
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

fn send_message(message: String) {
    let config = read_config();
    let token = config["TELEGRAM_TOKEN"].clone();
    let channel = config["TELEGRAM_CHANNEL"].clone();
    let endpoint = get_telegram_endpoint(token);

    ureq::post(endpoint.as_str())
        .send_form(&[
            ("chat_id", channel.as_str()),
            (
                "text",
                format!("I've been on for *{}*\\.", message).as_str(),
            ),
            ("parse_mode", "MarkdownV2"),
        ])
        .expect(ERROR_REQUEST_FAILED);
}

fn main() {
    send_message(get_duration_message(get_uptime()))
}
