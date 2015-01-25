package main

import (
	"fmt"
)

func main() {
	// fmt.Printf("primes : %v\n", primes(1000000))
	var result int64 = 0
	primes := primes(100000)
	for _, j := range primes {
		result += j
	}
	fmt.Printf("Primes: %v \nResult: %d\n", primes, result)
}

// primes gives the list of primest up to n-1 inclusive
// Uses the sieve of erastophenes
// int64 used Range: -9223372036854775808 through 9223372036854775807
func primes(n int64) []int64 {
	var primes []int64 = make([]int64, n+1)
	var i, j int64
	for i, j = 0, 0; i < int64(len(primes)); i++ {
		primes[i] = j
		j++
	}
	// fmt.Printf("v: %v\n", primes)
	var p int64 = 2
Outer:
	for true {
		var j int64
		for j = p + p; j < int64(len(primes)); j += p {
			primes[j] = 0
			// fmt.Printf("p: %d, i: %d, v: %v\n", p, j, primes)
		}
		for i := 0; i < len(primes); i++ {
			if primes[i] > p {
				p = primes[i]
				break
			}
			if i == len(primes)-1 {
				break Outer
			}
		}
	}

	primes[1] = 0
	for i := 0; i < len(primes); i++ {
		if primes[i] == 0 {
			primes = append(primes[:i], primes[i+1:]...)
			i -= 1
		}
	}
	return primes
}
