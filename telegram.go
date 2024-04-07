package main

import (
	"fmt"
	"log"
	"net/http"
	"net/url"
)

const ErrorRequestFailed = "request to Telegram failed"

func sendMessage(token, channel, message string) {
	endpoint := fmt.Sprintf("https://api.telegram.org/bot%s/sendMessage", token)
	form := url.Values{
		"chat_id":    {channel},
		"text":       {message},
		"parse_mode": {"MarkdownV2"},
	}

	_, err := http.PostForm(endpoint, form)
	if err != nil {
		log.Fatalln(ErrorRequestFailed)
	}
}
