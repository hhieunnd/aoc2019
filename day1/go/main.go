package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Result struct {
	Part1 int
	Part2 int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %s", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var result Result

	for scanner.Scan() {
		line := scanner.Text()

		num, err := strconv.Atoi(line)
		if err != nil {
			log.Fatalf("invalid number in input %s", err)
		}

		result.Part1 += fuelForMass(num)
		result.Part2 += totalFuel(num)
	}

	fmt.Printf("sum 1: %d \nsum 2: %d \n", result.Part1, result.Part2)
}

func fuelForMass(mass int) int {
	return mass/3 - 2
}

func totalFuel(num int) int {
	result := 0
	for {
		num = fuelForMass(num)
		if num < 0 {
			break
		}

		result += num
	}

	return result
}
