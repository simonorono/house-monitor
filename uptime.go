package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const (
	ErrorUnableToReadFile = "couldn't read uptime file"
	ErrorUptimeNotFloat   = "uptime value is not decimal"
)

func getUptime() uint64 {
	bytes, err := os.ReadFile("/proc/uptime")
	if err != nil {
		log.Fatalln(ErrorUnableToReadFile)
	}

	contents := string(bytes[:])
	parts := strings.Split(contents, " ")

	float, err := strconv.ParseFloat(parts[0], 64)
	if err != nil {
		log.Fatalln(ErrorUptimeNotFloat)
	}

	return uint64(float)
}

func getUptimeMessage() string {
	seconds := getUptime()

	values := []struct {
		uint64
		string
	}{
		{seconds / 60 / 60 / 24, "day"},
		{seconds / 60 / 60 % 24, "hour"},
		{seconds / 60 % 60, "minute"},
		{seconds % 60, "second"},
	}

	var toReturn []string

	for _, value := range values {
		if value.uint64 == 0 {
			continue
		}

		if value.uint64 > 1 {
			value.string += "s"
		}

		toReturn = append(toReturn, fmt.Sprintf("%d %s", value.uint64, value.string))
	}

	return strings.Join(toReturn, " ")
}
