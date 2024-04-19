const ERROR_REQUEST_FAILED: &str = "request to Telegram failed";

pub fn send_message(token: String, channel: String, message: String) {
    let endpoint = format!("https://api.telegram.org/bot{}/sendMessage", token);

    ureq::post(endpoint.as_str())
        .send_form(&[
            ("chat_id", channel.as_str()),
            ("text", message.as_str()),
            ("parse_mode", "MarkdownV2"),
        ])
        .expect(ERROR_REQUEST_FAILED);
}
