package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type instruction struct {
	direction string
	amount    int
}

func check(e error) {
	if e != nil {
		os.Exit(1)
	}
}

func readLines(path string) ([]string, error) {
	file, err := os.Open(path)
	check(err)
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines, scanner.Err()
}

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		os.Exit(1)
	}

	lines, err := readLines(args[0])

	check(err)

	var instructions []instruction
	for _, element := range lines {
		elements := strings.Fields(element)
		i, _ := strconv.Atoi(elements[1])
		instr := instruction{direction: elements[0], amount: i}
		instructions = append(instructions, instr)
	}

	pos_x := 0
	pos_y := 0

	for _, instr := range instructions {
		switch instr.direction {
		case "forward":
			pos_x += instr.amount
		case "up":
			pos_y += instr.amount
		case "down":
			pos_y -= instr.amount
		}
	}

	fmt.Printf("%d * %d = %d\n", pos_x, pos_y, pos_x*pos_y*(-1))

	pos_x = 0
	pos_y = 0
	aim := 0

	for _, instr := range instructions {
		switch instr.direction {
		case "forward":
			pos_x += instr.amount
			pos_y -= aim * instr.amount
		case "up":
			aim -= instr.amount
		case "down":
			aim += instr.amount
		}
	}

	fmt.Printf("%d * %d = %d\n", pos_x, pos_y, pos_x*pos_y*(-1))
}
