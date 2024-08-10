use std::collections::HashMap;
use reqwest::blocking::Client;

const ERROR_REQUEST_FAILED: &str = "request to Telegram failed";

pub fn send_message(webhook_url: String, message: String) {
    let mut request = HashMap::new();
    request.insert("content", message);

    Client::new()
        .post(webhook_url)
        .json(&request)
        .send()
        .expect(ERROR_REQUEST_FAILED);
}
