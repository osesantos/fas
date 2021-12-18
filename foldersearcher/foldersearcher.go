package foldersearcher

import (
	"fas/model"
	"io/fs"
	"io/ioutil"
	"log"
	"strings"
)

func SearchFolder(root string, search string) []model.File {
	files, err := ioutil.ReadDir(root)
	if err != nil {
		log.Fatal(err)
	}

	var filteredList []model.File

	if strings.Compare(search, "*") != 0 {
		filteredList = Filter(model.File{Path: root, Name: "."}, ConvertToFile(files, root), search)
	} else {
		filteredList = ConvertToFile(files, root)
	}

	return filteredList
}

func ConvertToFile(fi []fs.FileInfo, path string) []model.File {
	var array []model.File
	for _, s := range fi {
		file := model.File{Path: path, Name: s.Name()}
		array = append(array, file)
	}
	return array
}

func Contains(filesList []string, value string) bool {
	result := false
	for _, s := range filesList {
		if strings.Contains(s, value) {
			result = true
		}
	}
	return result
}

func Filter(root model.File, filesList []model.File, value string) []model.File {
	var array []model.File
	for _, f := range filesList {
		if strings.Contains(f.Name, value) {
			array = append(array, f)
		}
	}
	return array
}

type ModelFiles []model.File

func (list ModelFiles) head() model.File {
	if len(list) == 0 {
		return model.File{}
	} else {
		return list[0]
	}
}

func (list ModelFiles) tail() []model.File {
	if len(list) == 0 {
		return []model.File{}
	} else {
		return list[1:]
	}
}
