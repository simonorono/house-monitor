def get_uptime() -> int:
    with open('/proc/uptime', 'r') as file:
        content = file.read()

    if content is not None:
        return int(float(content.split(" ")[0]))
    else:
        raise "Couldn't read uptime from /proc/uptime"


if __name__ == '__main__':
    print(get_uptime())
