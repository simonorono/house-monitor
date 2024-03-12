const ERROR_REQUEST_FAILED: &str = "request to Telegram failed";

fn get_telegram_endpoint(token: String) -> String {
    format!("https://api.telegram.org/bot{}/sendMessage", token)
}

pub fn send_message(token: String, channel: String, message: String) {
    ureq::post(get_telegram_endpoint(token).as_str())
        .send_form(&[
            ("chat_id", channel.as_str()),
            ("text", message.as_str()),
            ("parse_mode", "MarkdownV2"),
        ])
        .expect(ERROR_REQUEST_FAILED);
}
