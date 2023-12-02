package main

import (
	"log"
	"os"
	"strings"
)

func getDuplicateIdx(char byte, arr *[]byte) int {
	// lastDplIdx := -1
	// for idx, n := range *arr {
	// 	if n == char {
	// 		lastDplIdx = idx
	// 	}
	// }
	//    log.Print(lastDplIdx, string(*arr))
	return strings.LastIndexAny(string(*arr), string(char))
	// return lastDplIdx
}

const max_chars = 14

func getMarker(packet string) int {
	fourDiff := make([]byte, 0, max_chars)
	for i := 0; i < len(packet); i++ {
		dplIdx := getDuplicateIdx(packet[i], &fourDiff)

		if dplIdx > -1 {
			if dplIdx+1 >= max_chars {
				fourDiff = make([]byte, 0, max_chars)
			} else {
				fourDiff = fourDiff[dplIdx+1:]
			}
		}
		if len(fourDiff) == max_chars {
			return i
		}
		if len(fourDiff) < max_chars {
			fourDiff = append(fourDiff, packet[i])
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
func doesContain(char byte, arr *[]byte) bool {
	for _, n := range *arr {
		if n == char {
			return true
		}
	}
	return false
}
