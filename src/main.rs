mod telegram;
mod uptime;

use telegram::send_message;
use uptime::get_duration_message;

fn main() {
    send_message(format!("I've been on for *{}*\\.", get_duration_message()));
}
