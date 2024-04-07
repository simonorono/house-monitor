package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path"
	"strings"
)

const (
	ErrorConfigFileUnreadable = "couldn't read config file"
	ErrorHomeVariableNotSet   = "couldn't find home env variable"
	ErrorMissingConfigValue   = "missing config value for key %s"
)

func readConfig() map[string]string {
	home, exists := os.LookupEnv("HOME")
	if !exists {
		log.Fatalln(ErrorHomeVariableNotSet)
	}

	filePath := path.Join(home, ".config", "house-monitor.conf")

	file, err := os.Open(filePath)
	if err != nil {
		log.Fatalln(ErrorConfigFileUnreadable)
	}

	config := make(map[string]string)
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		parts := strings.Split(line, "=")
		config[parts[0]] = parts[1]
	}

	for _, property := range [...]string{"DEVICE_NAME", "TELEGRAM_CHANNEL", "TELEGRAM_TOKEN"} {
		if _, ok := config[property]; !ok {
			log.Fatalf(ErrorMissingConfigValue, property)
		}
	}

	return config
}

func main() {
	config := readConfig()

	sendMessage(
		config["TELEGRAM_TOKEN"],
		config["TELEGRAM_CHANNEL"],
		fmt.Sprintf(
			"*\\[%s\\]* Uptime: *%s*\\.",
			config["DEVICE_NAME"],
			getUptimeMessage(),
		),
	)
}
