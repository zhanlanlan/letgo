package main

import (
	"log"
	"testing"
)

func TestIsValid(t *testing.T) {

	s := "()"
	log.Println(isValid(s))

	s = "()[]{}"
	log.Println(isValid(s))

	s = "(]"
	log.Println(isValid(s))

	s = "([)]"
	log.Println(isValid(s))

	s = "{[]}"
	log.Println(isValid(s))

}
