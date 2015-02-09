package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Printf("%v\n", collatzSeq(13))
	fmt.Printf("%v\n", collatzSeq(1000000))
	start := time.Now()
	var max int
	var maxLength int
	for i := 1000000; i > 1; i-- {
		currentLength := len(collatzSeq(i))
		if currentLength > maxLength {
			maxLength = currentLength
			max = i
		}
	}
	fmt.Printf("Max : %d, Max Length : %d\n", max, maxLength)
	fmt.Printf("%v\n", time.Since(start))
}

func collatzSeq(n int) []int {
	var seq []int
	i := n
	for i != 1 {
		seq = append(seq, i)
		if i%2 == 0 {
			i = i / 2
		} else {
			i = 3*i + 1
		}
	}
	seq = append(seq, i)
	return seq
}
