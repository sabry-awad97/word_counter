package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
)

func main() {
	// Define flags for counting lines, bytes, and runes
	l := flag.Bool("l", false, "count lines")

	b := flag.Bool("b", false, "count bytes")

	r := flag.Bool("r", false, "count runes")

	// Parsing the flags provided by the user
	flag.Parse()

	// Calling the count function to count the number of words​
	// received from the Standard Input
	counter := count(os.Stdin, *l, *b, *r)

	// // Print the final count value
	fmt.Println(counter)
}

func count(r io.Reader, countLines, countBytes, countRunes bool) int {
	// Initialize the counter and scanner
	var counter int
	scanner := bufio.NewScanner(r)

	switch {
	case countLines:
		scanner.Split(bufio.ScanLines)
	case countBytes:
		scanner.Split(bufio.ScanBytes)
	case countRunes:
		scanner.Split(bufio.ScanRunes)
	default:
		scanner.Split(bufio.ScanWords)
	}

	// For every word scanned, increment the counter
	for scanner.Scan() {
		counter++
	}

	return counter
}

// set GOOS=windows && go build -o wc.exe
// echo "Word Counter" | .\wc.exe
