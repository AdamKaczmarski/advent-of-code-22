package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
	// "bufio"
	"os"
)

func main() {
	// file, err := os.Open("./day1/input_test.txt")
	// if err != nil {
	// 	fmt.Println(err)
	// 	return
	// }
	// defer file.Close()
	// scanner := bufio.NewScanner(file)
	// for scanner.Scan() {
	// 	line := scanner.Text()
	//        fmt.Println(line)
	// }
	//
	//    if err:= scanner.Err(); err!=nil{
	//        fmt.Println("Error reading from file: ",err)
	//    }
	content, err := os.ReadFile("./day1/input.txt")
	if err != nil {
		fmt.Println("couldn't read file")
		return
	}
	lines := string(content)
	sums := []int{}
	for _, v := range strings.Split(lines, "\n\n") {
		sum := 0
		for _, unit := range strings.Fields(v) {
			if len(v) > 0 {
				n, err := strconv.Atoi(unit)
				if err != nil {
					fmt.Printf("couldn't parse %q", unit)
				}
				sum += n
			}
		}
		sums = append(sums, sum)
	}
	slices.Sort(sums)
    sums = sums[len(sums)-3:]
    fmt.Println(sums)
	top3 := 0
	for _, v := range sums {
        top3+=v
	}
    fmt.Println(top3)

}
