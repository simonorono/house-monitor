use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

use telegram::send_message;
use uptime::get_duration_message;

mod telegram;
mod uptime;

const ERROR_CONFIG_FILE_UNREADABLE: &str = "couldn't read config file";
const ERROR_HOME_VARIABLE_NOT_SET: &str = "couldn't find home env variable";

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

fn main() {
    let config = read_config();

    send_message(
        config["TELEGRAM_TOKEN"].to_string(),
        config["TELEGRAM_CHANNEL"].to_string(),
        format!(
            "*\\[{}\\]* Uptime: *{}*\\.",
            config["DEVICE_NAME"],
            get_duration_message(),
        ),
    );
}
