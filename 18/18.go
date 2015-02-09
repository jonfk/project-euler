package main

import (
	// "fmt"
	// "github.com/davecgh/go-spew/spew"
	"log"
	"strconv"
	"strings"
)

const input = `75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23`

type Node struct {
	Value       int
	Left, Right *Node
}

func main() {
	// fmt.Printf("%#v\n", graph)
	graph := load()
	// spew.Dump(graph)
}

func load() *Node {
	var lastLayer []*Node
	var newLayer []*Node
	var first *Node
	lines := strings.Split(input, "\n")
	// fmt.Printf("%#v\n", lines)
	for i := range lines {
		line := strings.Split(lines[i], " ")
		for j := range line {
			num, err := strconv.Atoi(line[j])
			if err != nil {
				log.Fatal(err)
			}
			node := &Node{Value: num}
			if lastLayer == nil {
				first = node
			}
			newLayer = append(newLayer, node)
		}
		setPrevIndex(lastLayer, newLayer)
		lastLayer = newLayer
		newLayer = nil
		// fmt.Printf("%#v\n", line)
	}
	return first
}

func setPrevIndex(prev, cur []*Node) {
	for i := range prev {
		prev[i].Left = cur[i]
		prev[i].Right = cur[i+1]
	}

}
