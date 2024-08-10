#!/bin/env python

import json
import os
import sys
import time
from http.client import HTTPSConnection
from urllib.parse import urlparse

CONFIG_FILE = f'{os.environ['HOME']}/.config/house-monitor.conf'
REQUIRED_PROPERTIES = ['DEVICE_NAME', 'DISCORD_WEBHOOK']
UPTIME_FILE = '/proc/uptime'


def read_config():
    config = {}

    with open(CONFIG_FILE, 'r') as f:
        for line in f.readlines():
            (key, value) = line.strip().split('=')
            config[key] = value

    for prop in REQUIRED_PROPERTIES:
        if not prop in config.keys():
            print(f"Required property '{prop}' not found in config file")
            sys.exit(1)

    return config


def get_message():
    with open(UPTIME_FILE, 'r') as f:
        seconds = int(float(f.read().split(' ')[0]))

        units = [
            (seconds // 60 // 60 // 24, "day"),
            (seconds // 60 // 60 % 24, "hour"),
            (seconds // 60 % 60, "minute"),
            (seconds % 60, "second"),
        ]

        return ' '.join([
            f'{x} {y if x == 1 else f'{y}s'}'
            for (x, y) in units
            if x > 0
        ])


def send_message(webhook, message):
    url = urlparse(webhook)
    conn = HTTPSConnection(url.netloc)

    conn.request(
        'POST',
        url.path,
        json.dumps({'content': message}),
        {'Content-Type': 'application/json'}
    )

    conn.getresponse()


def main():
    timestamp = int(time.time())
    config = read_config()

    send_message(
        config['DISCORD_WEBHOOK'],
        f'**[<t:{timestamp}>][{config['DEVICE_NAME']}]** Uptime: {get_message()}'
    )


if __name__ == '__main__':
    main()
