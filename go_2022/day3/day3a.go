package main

import (
	"log"
	"os"
	"strings"
)

func findMutualItem(fC, sC string) byte {
	counter := make(map[byte]int)
	for i := 0; i < len(fC); i++ {
		counter[fC[i]] = 1
	}
	for i := 0; i < len(sC); i++ {
		if _, ok := counter[sC[i]]; ok {
			return sC[i]
		}
	}
	return 0
}

func calculatePriority(character byte) int {
	if character > 'a' {
		return int(character - 'a' + 1)
	} else {
		return int(character - 'A' + 27)
	}
}

func main() {
	fileBytes, err := os.ReadFile("./day3/input.txt")
	if err != nil {
		log.Fatal(err)
		return
	}
	lines := string(fileBytes)
	sum := 0
	for _, line := range strings.Fields(lines) {
		firstCompartment := line[0 : len(line)/2]
		secondCompartment := line[len(line)/2:]
		mutualItem := findMutualItem(firstCompartment, secondCompartment)
		sum += calculatePriority(mutualItem)
	}
	log.Printf("Priority sum: %v", sum)
}
