package foldersearcher

import (
	"fmt"
	"io/fs"
	"io/ioutil"
	"log"
)

func SearchFolder(root string) []string {
	files, err := ioutil.ReadDir(root)
	if err != nil {
		log.Fatal(err)
	}
	for _, file := range files {
		fmt.Println(file.Name())
	}

	return files.convertToString()
}

func (fi *[]fs.FileInfo) convertToString() []string {
	array := nil
	for _, s := range fi {
		array.add(s.Name())
	}
	return array
}
