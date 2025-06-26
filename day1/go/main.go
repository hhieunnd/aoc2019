package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %s", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// part 1
	sum1 := 0

	// part 2
	sum2 := 0

	for scanner.Scan() {
		line := scanner.Text()

		num, _ := strconv.Atoi(line)

		sum1 += (num/3 - 2)
		sum2 += countToZero(num)
	}

	fmt.Printf("sum 1: %d \n", sum1)
	fmt.Printf("sum 2: %d \n", sum2)
}

func countToZero(num int) int {
	result := 0
	for num > 0 {
		num = num/3 - 2
		if num < 0 {
			num = 0
		}

		result += num
	}

	return result
}
