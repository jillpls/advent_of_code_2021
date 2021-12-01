package main

import (
	"bufio"
	"os"
	"strconv"
)

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

	var number_lines []int
	for _, element := range lines {
		i, _ := strconv.Atoi(element)
		number_lines = append(number_lines, i)
	}

	previousElement := number_lines[0]
	increases := 0

	for _, e := range number_lines {
		if e > previousElement {
			increases = increases + 1
		}
		previousElement = e
	}

	println(increases)

	increases = 0
	window := []int{number_lines[0], number_lines[1], number_lines[2]}
	i := 3

	for i < len(number_lines) {
		new_window := window[1:]
		new_window = append(new_window, number_lines[i])
		window_size := window[0] + window[1] + window[2]
		new_window_size := new_window[0] + new_window[1] + new_window[2]
		if new_window_size > window_size {
			increases += 1
		}
		window = new_window
		i += 1
	}

	println(increases)
}
