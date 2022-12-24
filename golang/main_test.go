package main

import (
	"bytes"
	"strings"
	"testing"
	"unicode/utf8"
)

func countWords(s string) int {
	words := strings.Fields(s)
	return len(words)
}

func countLines(s string) int {
	lines := strings.Count(s, "\n")
	if len(s) > 0 && !strings.HasSuffix(s, "\n") {
		lines++
	}
	return lines
}

func countBytes(s string) int {
	return len(s)
}

func countRunes(s string) int {
	return utf8.RuneCountInString(s)
}

func TestCountWords(t *testing.T) {
	// Arrange
	words := "hello world\n"
	expected := countWords(words)

	// Act
	b := bytes.NewBufferString(words)
	result := count(b, false, false, false)

	// Assert
	if result != expected {
		t.Errorf("%v: expected %v got %v instead", words, expected, result)
	}
}

func TestCountLines(t *testing.T) {
	// Arrange
	words := "hello world\n"
	expected := countLines(words)

	// Act
	b := bytes.NewBufferString(words)
	result := count(b, true, false, false)

	// Assert
	if result != expected {
		t.Errorf("%v: expected %v got %v instead", words, expected, result)
	}
}

func TestCountBytes(t *testing.T) {
	// Arrange
	s := "We♥Go" // 7
	expected := countBytes(s)

	// Act
	b := bytes.NewBufferString(s)
	result := count(b, false, true, false)

	// Assert
	if result != expected {
		t.Errorf("%v: expected %v got %v", s, expected, result)
	}
}

func TestCountRunes(t *testing.T) {
	// Arrange
	s := "We♥Go" // 5
	expected := countRunes(s)

	// Act
	b := bytes.NewBufferString(s)
	result := count(b, false, false, true)

	// Assert
	if result != expected {
		t.Errorf("%v: expected %v got %v", s, expected, result)
	}
}
