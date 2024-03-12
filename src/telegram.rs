use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

const ERROR_CONFIG_FILE_UNREADABLE: &str = "couldn't read config file";
const ERROR_HOME_VARIABLE_NOT_SET: &str = "couldn't find home env variable";
const ERROR_REQUEST_FAILED: &str = "request to Telegram failed";

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

pub fn send_message(message: String) {
    let config = read_config();
    let token = config["TELEGRAM_TOKEN"].clone();
    let channel = config["TELEGRAM_CHANNEL"].clone();
    let endpoint = get_telegram_endpoint(token);

    ureq::post(endpoint.as_str())
        .send_form(&[
            ("chat_id", channel.as_str()),
            ("text", message.as_str()),
            ("parse_mode", "MarkdownV2"),
        ])
        .expect(ERROR_REQUEST_FAILED);
}
