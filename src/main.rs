mod telegram;
mod uptime;

use telegram::send_message;
use uptime::get_duration_message;

fn main() {
    send_message(get_duration_message());
}
