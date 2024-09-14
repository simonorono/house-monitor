House Monitor
============

Utility to post uptime of any Linux system to a Discord channel via webhooks.

Tested with Go 1.23.0 running on Linux.

## Configuration

Config file must be placed in `$HOME/.config/house-monitor.json`. This file
needs the following values:

* `DEVICE_NAME`: appears on the message sent
* `DISCORD_WEBHOOK`: specific to the channel we want to send the messages to

This file must be in .env format.

#### Learn more about Discord webhooks [here](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) and [here](https://discord.com/developers/docs/resources/webhook).
