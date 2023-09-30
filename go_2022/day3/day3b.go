package main

import (
	"log"
	"os"
	"strings"
)

func findMutualItem(fG, sG, tG string) byte {
	counter := make(map[byte]int)
	for i := 0; i < len(fG); i++ {
		if _, ok := counter[fG[i]]; ok {
			continue
		} else {
			counter[fG[i]]++
		}
	}
	for i := 0; i < len(sG); i++ {
		if _, ok := counter[sG[i]]; ok {
			if counter[sG[i]] == 2 {
				continue
			} else {
				counter[sG[i]]++
			}
		}
	}
	for i := 0; i < len(tG); i++ {
		if v, ok := counter[tG[i]]; ok && v == 2 {
			return tG[i]
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
	groups := strings.Fields(lines)
	for i := 0; i < len(groups); i += 3 {
		group := groups[i : i+3]
		sum += calculatePriority(findMutualItem(group[0], group[1], group[2]))
	}
	log.Printf("Priority sum: %v", sum)
}
