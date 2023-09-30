package main

import (
	"log"
	"os"
	"strings"
)

const (
	Lose = 0
	Draw = 3
	Win  = 6

	Rock     = 1
	Paper    = 2
	Scissors = 3
)

var choices = map[string]int{
	"X": Lose,
	"Y": Draw,
	"Z": Win,
}

func calculateScore(enemy, me string) (result int) {
	switch enemy {
	case "A":
		switch me {
		case "X":
			result += Scissors 
		case "Y":
			result += Rock 
		case "Z":
			result += Paper 
		}
	case "B":
		{
			switch me {
			case "X":
				result += Rock 
			case "Y":
				result += Paper 
			case "Z":
				result += Scissors 
			}
		}
	case "C":
		{
			switch me {
			case "X":
				result += Paper
			case "Y":
				result += Scissors
			case "Z":
				result += Rock
			}

		}
	}
	result += choices[me]
	return result
}

func main() {

	fileContent, err := os.ReadFile("./day2/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := string(fileContent)
	var score int = 0
	for _, line := range strings.Split(lines, "\n") {
		match := strings.Split(line, " ")
		if len(match) == 2 {
            // log.Printf("%q", match)
			score += calculateScore(match[0], match[1])
		}
	}
	log.Printf("Score: %v", score)
}
