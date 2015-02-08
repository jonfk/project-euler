package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	numbers := load("11.txt")
	var max int
	for i := range numbers {
		for j := range numbers[i] {
			candidate := testProduct(i, j, numbers)
			if candidate > max {
				max = candidate
			}
		}
	}
	fmt.Printf("RESULT %d\n", max)
}

func product(a, b, c, d int) int {
	return a * b * c * d
}

func testProduct(row, col int, matrix [][]int) int {
	var test [8]int
	if col > 2 {
		test[0] = matrix[row][col] * matrix[row][col-1] * matrix[row][col-2] * matrix[row][col-3]
	}
	if col < len(matrix[row])-4 {
		test[1] = matrix[row][col] * matrix[row][col+1] * matrix[row][col+2] * matrix[row][col+3]
	}
	if row > 2 {
		test[2] = matrix[row][col] * matrix[row-1][col] * matrix[row-2][col] * matrix[row-3][col]
	}
	if row < len(matrix)-4 {
		test[3] = matrix[row][col] * matrix[row+1][col] * matrix[row+2][col] * matrix[row+3][col]
	}
	if col > 2 && row > 2 {
		test[4] = matrix[row][col] * matrix[row-1][col-1] * matrix[row-2][col-2] * matrix[row-3][col-3]
	}
	if col < len(matrix[row])-4 && row < len(matrix)-4 {
		test[5] = matrix[row][col] * matrix[row+1][col+1] * matrix[row+2][col+2] * matrix[row+3][col+3]
	}
	if col > 2 && row < len(matrix)-4 {
		test[6] = matrix[row][col] * matrix[row+1][col-1] * matrix[row+2][col-2] * matrix[row+3][col-3]
	}
	if col < len(matrix[row])-4 && row > 2 {
		test[7] = matrix[row][col] * matrix[row-1][col+1] * matrix[row-2][col+2] * matrix[row-3][col+3]
	}
	return max(test)
}

func max(nums [8]int) int {
	var max int
	for i := range nums {
		if nums[i] > max {
			max = nums[i]
		}
	}
	return max
}

func load(filename string) [][]int {
	var result [][]int
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	fileBytes, err := ioutil.ReadAll(file)
	if err != nil {
		log.Fatal(err)
	}
	fileString := string(fileBytes)
	lines := strings.Split(fileString, "\n")
	for i := range lines {
		if lines[i] == "" {
			continue
		}
		digits := strings.Split(lines[i], " ")
		var row []int
		for j := range digits {
			digit, err := strconv.Atoi(digits[j])
			if err != nil {
				log.Fatal(err)
			}
			row = append(row, digit)
		}
		result = append(result, row)
	}
	return result
}
