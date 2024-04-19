use reqwest::blocking::Client;
use std::collections::HashMap;

const ERROR_REQUEST_FAILED: &str = "request to Telegram failed";

pub fn send_message(token: String, channel: String, message: String) {
    let endpoint = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let mut request = HashMap::new();
    request.insert("chat_id", channel);
    request.insert("text", message.replace('.', "\\."));
    request.insert("parse_mode", "MarkdownV2".to_string());

    Client::new()
        .post(endpoint)
        .json(&request)
        .send()
        .expect(ERROR_REQUEST_FAILED);
}
