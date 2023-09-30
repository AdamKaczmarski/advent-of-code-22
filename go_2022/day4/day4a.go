package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func convertToInt(s string) int {
	res, err := strconv.Atoi(s)
	if err != nil {
		log.Fatalf("Couldn't parse %v ", s, err)
	}
	return res
}

func getBounds(declaration string) (x, y int) {
	parts := strings.Split(declaration, "-")
	x, y = convertToInt(parts[0]), convertToInt(parts[1])
	return
}

func doesContain(x1, y1, x2, y2 int) bool {
    return x1 <= x2 && y1 >= y2 
}

func main() {
	fileContent, err := os.ReadFile("./day4/input.prod")
	if err != nil {
		log.Fatal(err)
	}
	lines := string(fileContent)
	count := 0
	for _, line := range strings.Fields(lines) {
		assignedSections := strings.Split(line, ",")
		x1, y1 := getBounds(assignedSections[0])
		x2, y2 := getBounds(assignedSections[1])
		if doesContain(x1, y1, x2, y2) || doesContain(x2, y2, x1, y1) {
			count++
		}
	}
	log.Printf("Fully contained pairs %v", count)
}
