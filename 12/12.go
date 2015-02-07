package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Printf("test : %d\n", countDivisors(10000))
	start := time.Now()
	lastTri := 0
	for i := 1; ; i++ {
		lastTri = newTri(i, lastTri)
		numDivisors := countDivisors(lastTri)
		if numDivisors >= 500 {
			fmt.Printf("RESULT: %d, Divisors: %d\n\n", lastTri, numDivisors)
			acceptor <- lastTri
			break
		}
	}
	elapsed := time.Since(start)
	fmt.Printf("Time taken : %s\n", elapsed)
}

func triangle(nth int) int {
	var result int
	for i := 0; i <= nth; i++ {
		result += i
	}
	return result
}

func newTri(nth, last int) int {
	return last + nth
}

func divisors(n int) []int {
	var divisors []int

	for i := 1; i <= n; i++ {
		if n%i == 0 {
			divisors = append(divisors, i)
		}
	}
	return divisors
}

func countDivisors(n int) int {
	var divisors int

	for i := 1; i <= (n / 2); i++ {
		if n%i == 0 {
			divisors += 1
		}
	}
	// fmt.Printf("divisors : %d for %d\n", divisors, n)
	return divisors + 1
}
