package main

import (
	"fmt"
	"math/big"
	"strconv"
)

func main() {
	fmt.Printf("%d\n", sum(fac(100).String()))
}

func fac(n int) *big.Int {
	var result *big.Int = big.NewInt(1)
	for i := n; i > 0; i-- {
		result.Mul(result, big.NewInt(int64(i)))
	}
	return result
}

func sum(number string) int {
	var result int
	for i := 0; i < len(number); i++ {
		n, _ := strconv.Atoi(string(number[i]))
		result += n
	}
	return result
}
