import json
import os
from http.client import HTTPSConnection


def read_config() -> dict[str, str]:
    path = os.path.join(os.environ['HOME'], '.config', 'house-monitor.conf')
    config = {}

    with open(path, 'r') as file:
        for line in file.readlines():
            key, value = line.split('=')
            config[key.strip()] = value.strip()

    return config


def get_uptime() -> int:
    with open('/proc/uptime', 'r') as file:
        content = file.read()

    if content is not None:
        return int(float(content.split(" ")[0]))
    else:
        raise "Couldn't read uptime from /proc/uptime"


def get_duration_message(seconds: int) -> str:
    parts = [
        [seconds // 60 // 60 // 24, "day"],
        [seconds // 60 // 60 % 24, "hour"],
        [seconds // 60 % 60, "minute"],
        [seconds % 60, "second"]
    ]

    parts = [x for x in parts if x[0] > 0]

    if len(parts) == 0:
        return "1 second"

    for part in parts:
        if part[0] != 1:
            part[1] = f"{part[1]}s"

    return ' '.join([f'{p[0]} {p[1]}' for p in parts])


def send_message(message: str):
    config = read_config()

    path = f"/bot{config['TELEGRAM_TOKEN']}/sendMessage"

    conn = HTTPSConnection("api.telegram.org")

    body = {
        'chat_id': config['TELEGRAM_CHANNEL'],
        'text': f"I've been on for *{message}*\\.",
        'parse_mode': 'MarkdownV2',
    }

    headers = {'content-type': 'application/json'}

    conn.request('POST', path, json.dumps(body), headers)
    conn.getresponse()
    conn.close()


if __name__ == '__main__':
    send_message(get_duration_message(get_uptime()))
