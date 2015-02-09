package main

import (
	"fmt"
	"math/big"
	"strconv"
)

func main() {
	two := big.NewInt(2)
	two.Exp(two, big.NewInt(1000), big.NewInt(0))

	fmt.Printf("%s\n", two)

	twoStr := two.String()

	var result int
	for i := 0; i < len(twoStr); i++ {
		num, _ := strconv.Atoi(string(twoStr[i]))
		result += num
	}
	fmt.Printf("RESULT: %d\n", result)
}
