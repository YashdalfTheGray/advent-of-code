// This file is generated by the setup script located in the utils folder in the repository root.
// Day 4
// https://adventofcode.com/2015/day/4
// input: day4/input.txt

package main

import (
	"crypto/md5"
	"errors"
	"fmt"
	"math"
	"strconv"
)

func Part1(input string) (int, error) {
	for i := 0; i <= math.MaxInt; i++ {
		hash := md5.Sum([]byte(input + strconv.Itoa(i)))
		firstFive := fmt.Sprintf("%x", hash)[:5]
		if firstFive == "00000" {
			return i, nil
		}
	}

	return 0, errors.New("No number found with hash starting with 00000")
}

func Part2(input string) (int, error) {
	for i := 0; i <= math.MaxInt; i++ {
		hash := md5.Sum([]byte(input + strconv.Itoa(i)))
		firstSix := fmt.Sprintf("%x", hash)[:6]
		if firstSix == "000000" {
			return i, nil
		}
	}

	return 0, errors.New("No number found with hash starting with 00000")
}

func main() {
	part1Output, part1Err := Part1("ckczppom")
	if part1Err != nil {
		fmt.Println(part1Err)
	} else {
		fmt.Println("Day 4 part 1 - ", part1Output)
	}

	part2Output, part2Err := Part2("ckczppom")
	if part2Err != nil {
		fmt.Println(part2Err)
	} else {
		fmt.Println("Day 4 part 2 - ", part2Output)
	}
}