package main

import (
	"log"
	"os"
	// "strconv"
	"strings"
)

func main() {
	fileContent, err := os.ReadFile("./day5/input.test")
	if err != nil {
		log.Fatal(err)
		return
	}
	lines := string(fileContent)
	for _, line := range strings.Split(lines,"\n") {
        log.Printf("%q",strings.Split(line," "))
	}
}
