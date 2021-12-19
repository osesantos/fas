package main

import (
	"fas/foldersearcher"
	"flag"
	"fmt"
	"strconv"

	"github.com/vutran/ansi/colors"
	"github.com/vutran/ansi/styles"
)

func main() {
	folder := flag.String("f", ".", "Specify the folder to search for")
	file := flag.String("ff", "*", "Specify the file to search for")
	flag.Parse()

	rootFolder := *folder
	fileSearch := *file

	// Todo :: do a recursive search and return path and file name
	filesList := foldersearcher.SearchFolder(rootFolder, fileSearch)

	if len(filesList) > 0 {
		fmt.Println("##########################################")
		fmt.Println(" pwd: " + rootFolder)
		fmt.Println("------------------------------------------")
		fmt.Println(" " + strconv.Itoa(len(filesList)) + " entries found for " + fileSearch)
		fmt.Println()

		for _, e := range filesList {
			boldName := styles.Bold(colors.Red(e.Name))
			fmt.Println("> " + e.Path + "/" + boldName)
		}
	}
}
