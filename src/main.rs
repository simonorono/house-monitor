use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

use clap::{Parser, Subcommand};

use discord::send_message;
use uptime::get_duration_message;

mod discord;
mod uptime;

const ERROR_CONFIG_FILE_UNREADABLE: &str = "couldn't read config file";
const ERROR_HOME_VARIABLE_NOT_SET: &str = "couldn't find home env variable";
const REQUIRED_CONFIG_VALUES: [&str; 2] = ["DEVICE_NAME", "DISCORD_WEBHOOK"];

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

    REQUIRED_CONFIG_VALUES.iter().for_each(|v| {
        if !config.contains_key(&v.to_string()) {
            panic!("Missing config value for key {}", v)
        }
    });

    config
}

#[derive(Subcommand)]
enum Commands {
    Uptime,
    Version,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    fn send_message(&self, message: String) {
        let config = read_config();

        send_message(
            config["DISCORD_WEBHOOK"].to_string(),
            format!("**[{}]** {}", config["DEVICE_NAME"], message),
        );
    }

    fn run(&self) {
        match self.command {
            Some(Commands::Uptime) => {
                self.send_message(format!("Uptime: {}", get_duration_message()))
            }
            Some(Commands::Version) => {
                self.send_message(format!("Version: {}", env!("CARGO_PKG_VERSION")))
            }
            None => println!("nothing to do"),
        }
    }
}

fn main() {
    Cli::parse().run()
}
