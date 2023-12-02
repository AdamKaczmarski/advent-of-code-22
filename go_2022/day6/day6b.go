package main

import (
	"log"
	"os"
	"strings"
)

func duplicateIndex(char byte, arr *[]byte) int {
	lastDplIdx := -1
	for idx, n := range *arr {
		if n == char {
			lastDplIdx = idx
		}
	}
	return lastDplIdx
}

func getMarker(packet string) int {
	fourDiff := make([]byte, 0, 4)
	for i := 0; i < len(packet); i++ {
		log.Println(string(fourDiff))
		idx := strings.LastIndex(string(fourDiff), string(packet[i]))
		if len(fourDiff) < 4 {
			fourDiff = append(fourDiff, packet[i])
		}
		log.Println("last log", idx, string(packet[i]))
		if idx > 0 {
            right := fourDiff[len(fourDiff)-idx:]
            fourDiff = right
			// fourDiff = fourDiff[idx:]
            log.Println("After slice", string(fourDiff))
			// fourDiff = append(fourDiff, packet[i])
		}
		if len(fourDiff) == 4 {
            log.Println(string(fourDiff))
			return i+1
		}
	}
	return -1
}

func main() {
	fileContent, err := os.ReadFile("./day6/input.test")
	if err != nil {
		log.Fatal(err)
		return
	}
	lines := string(fileContent)
	for _, line := range strings.Fields(lines) {
		log.Println(line, getMarker(line))
	}
}
