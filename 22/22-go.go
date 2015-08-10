package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"sort"
)

type Names struct {
	Names []string
}

type Tuple2 struct {
	Name  string
	Score int
}

type Tuple2s []Tuple2

func (a Tuple2s) Len() int {
	return len(a)
}

func (a Tuple2s) Less(i, j int) bool {
	return a[i].Name < a[j].Name
}

func (a Tuple2s) Swap(i, j int) {
	a[i], a[j] = a[j], a[i]
}

func main() {

	var names Names
	b, err := ioutil.ReadFile("./names.txt")
	if err != nil {
		log.Fatal(err)
	}
	json.Unmarshal(b, &names)

	var scoresTable map[string]int = make(map[string]int)

	for i := range names.Names {
		scoresTable[names.Names[i]] = score(names.Names[i])
	}

	var namesRanks Tuple2s

	for key, val := range scoresTable {
		namesRanks = append(namesRanks, Tuple2{key, val})
	}
	sort.Sort(namesRanks)

	var ranksTable map[string]int = make(map[string]int)

	for i := range namesRanks {
		name := namesRanks[i].Name
		ranksTable[name] = i + 1
	}

	var result int
	for i := range names.Names {
		val := scoresTable[names.Names[i]] * ranksTable[names.Names[i]]
		result += val
	}
	fmt.Printf("%d\n", result)

}

func score(name string) int {
	asRune := []rune(name)
	var s int

	for i := range asRune {
		s += int(asRune[i]) - 64
	}
	return s
}
