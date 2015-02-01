package main

import (
	"bytes"
	"fmt"
	"math"
)

func main() {
	var buf bytes.Buffer
	for i := 1; i <= 1000; i++ {
		buf.WriteString(num2Str(i))
	}
	fmt.Printf("Result: %d\n", len(buf.String()))
}

// Provides string for numbers up to 1000 inclusive
func num2Str(num int) string {
	numbers := map[int]string{
		1:    "one",
		2:    "two",
		3:    "three",
		4:    "four",
		5:    "five",
		6:    "six",
		7:    "seven",
		8:    "eight",
		9:    "nine",
		10:   "ten",
		11:   "eleven",
		12:   "twelve",
		20:   "twenty",
		30:   "thirty",
		40:   "forty",
		50:   "fifty",
		60:   "sixty",
		70:   "seventy",
		80:   "eighty",
		90:   "ninety",
		1000: "one thousand",
	}
	switch {
	case numbers[num] != "":
		return numbers[num]
	case num > 13 && num < 20:
		return numbers[digit(num, 1)] + "teen"
	case num < 100:
		return numbers[(digit(num, 2)*10)] + numbers[digit(num, 1)]
	case num > 100:
		return numbers[(num/100)] + "hundredand" + num2Str(num%100)
	}
	return ""
}

// Only works for positive integers in base 10
func digit(number, n int) int {
	return int((number / int(math.Pow10(n-1))) % 10)
}
