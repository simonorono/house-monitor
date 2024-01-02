House Monitor
============

Utility to post uptime of any Linux system to a Telegram channel.

Tested on Python 3.11.6

## Configuration

Config file must be placed in `$HOME/.config/house-monitor.conf`. This file only
needs two values:

* `TELEGRAM_TOKEN`: the authentication token for a Telegram bot.
* `TELEGRAM_CHANNEL`: the channel id for the channel the message will be posted to.

This file must be in .env format.

#### Learn more about Telegram bots [here](https://core.telegram.org/bots/tutorial).
