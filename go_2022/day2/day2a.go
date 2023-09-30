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
	"X": Rock,
	"Y": Paper,
	"Z": Scissors,
}

func calculateScore(enemy, me string) (result int) {
	switch enemy {
	case "A":
		switch me {
		case "X":
			result += Draw
		case "Y":
			result += Win
		case "Z":
			result += Lose
		}
	case "B":
		{
			switch me {
			case "X":
				result += Lose
			case "Y":
				result += Draw
			case "Z":
				result += Win
			}
		}
	case "C":
		{
			switch me {
			case "X":
				result += Win
			case "Y":
				result += Lose
			case "Z":
				result += Draw
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
			score += calculateScore(match[0], match[1])
		}
	}
	log.Printf("Score: %v", score)
}
