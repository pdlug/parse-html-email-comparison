package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

type Message struct {
	Id        string `json:"id"`
	FromEmail string `json:"fromEmail"`
	FromName  string `json:"fromName"`
	HtmlBody  string `json:"HtmlBody"`
}

func ExtractLinks(html string) []string {
	doc, err := goquery.NewDocumentFromReader(strings.NewReader(html))
	if err != nil {
		log.Fatal(err)
	}

	var links []string

	doc.Find("a").Each(func(i int, s *goquery.Selection) {
		target, exists := s.Attr("href")
		if exists {
			links = append(links, target)
		}
	})

	return links
}

func main() {
	start := time.Now()

	content, err := ioutil.ReadFile("../messages.json")
	if err != nil {
		fmt.Println(err)
	}

	var payload []Message
	err = json.Unmarshal(content, &payload)
	if err != nil {
		log.Fatal("Error during Unmarshal(): ", err)
	}

	var alllinks []string

	for _, value := range payload {
		alllinks = append(alllinks, ExtractLinks(value.HtmlBody)...)
	}

	ioutil.WriteFile("links.txt", []byte(strings.Join(alllinks, "\n")), 0644)

	duration := time.Since(start)
	fmt.Printf("time: %dms\n", duration.Milliseconds())
}
