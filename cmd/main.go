package main

import (
	"fas/foldersearcher"
	"flag"
	"fmt"
	"strings"
)

func main() {
	folder := flag.String("f", ".", "Specify the folder to search for.")
	file := flag.String("ff", "*", "Specify the file to search for.")
	flag.Parse()

	rootFolder := *folder
	fileSearch := *file

	filesList := foldersearcher.SearchFolder(rootFolder)

	if filesList.Contains(fileSearch) {
		fmt.Println("found: " + fileSearch)
	}
}

func (ss []strings) Contains(value string) bool {
	result := false
	for _, s := range ss {
		if strings.Contains(ss, value) {
			result = true
		}
	}
	return result
}
