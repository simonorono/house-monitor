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


if __name__ == '__main__':
    print(get_duration_message(get_uptime()))
