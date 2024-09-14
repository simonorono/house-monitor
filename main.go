package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

var config struct {
	Device  string `json:"device"`
	Webhook string `json:"webhook"`
}

func throwIf(err error, message string) {
	if err != nil {
		panic(message)
	}
}

func readConfig() {
	home := os.Getenv("HOME")
	path := filepath.Join(home, ".config/house-monitor.json")

	data, err := os.ReadFile(path)
	throwIf(err, "Could not read config file")

	err = json.Unmarshal(data, &config)
	throwIf(err, "Bad format on config file")
}

func systemUptime() int64 {
	data, err := os.ReadFile("/proc/uptime")
	throwIf(err, "Could not read system uptime")

	uptimeString := string(data)
	parts := strings.Split(uptimeString, " ")

	uptime, err := strconv.ParseFloat(parts[0], 64)
	throwIf(err, "Uptime not float")

	return int64(uptime)
}

func uptimeMessage() string {
	uptime := systemUptime()

	units := []struct {
		value int64
		unit  string
	}{
		{uptime / 60 / 60 / 24, "day"},
		{uptime / 60 / 60 % 24, "hour"},
		{uptime / 60 % 60, "minute"},
		{uptime % 60, "second"},
	}

	var messages []string

	for _, v := range units {
		if v.value == 0 {
			continue
		}

		if v.value > 1 {
			v.unit = fmt.Sprintf("%vs", v.unit)
		}

		messages = append(messages, fmt.Sprintf("%v %v", v.value, v.unit))
	}

	return strings.Join(messages, " ")
}

func sendMessage(content string) {
	data, err := json.Marshal(struct {
		Content string `json:"content"`
	}{content})
	throwIf(err, "Could not create request body")

	req, err := http.NewRequest("POST", config.Webhook, bytes.NewBuffer(data))
	throwIf(err, "Could not create request")

	req.Header.Set("Content-Type", "application/json")

	client := http.Client{}
	response, err := client.Do(req)
	throwIf(err, "Request failed")

	defer func(Body io.ReadCloser) {
		_ = Body.Close()
	}(response.Body)

	if (response.StatusCode-200) < 0 || (response.StatusCode-200) >= 100 {
		panic("Request didn't return successful status")
	}
}

func main() {
	readConfig()

	message := fmt.Sprintf(
		"**[%v][%v]** Uptime: %v",
		time.Now().Format(time.DateTime),
		config.Device,
		uptimeMessage(),
	)

	sendMessage(message)
}
