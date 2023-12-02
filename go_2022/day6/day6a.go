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

func getMarker(packet string) int {
	fourDiff := make([]byte, 0, 4)
	for i := 0; i < len(packet); i++ {
		dplIdx := getDuplicateIdx(packet[i], &fourDiff)

		if dplIdx > -1 {
			if dplIdx+1 >= 4 {
				fourDiff = make([]byte, 0, 4)
			} else {
				fourDiff = fourDiff[dplIdx+1:]
			}
		}
		if len(fourDiff) == 4 {
			return i
		}
		if len(fourDiff) < 4 {
			fourDiff = append(fourDiff, packet[i])
		}
	}
	return -1
}

func main() {
	fileContent, err := os.ReadFile("./day6/input.prod")
	if err != nil {
		log.Fatal(err)
		return
	}
	lines := string(fileContent)
	for _, line := range strings.Fields(lines) {
		log.Println(getMarker(line))
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
