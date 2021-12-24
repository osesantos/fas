#![allow(unused)]

use std::borrow::Borrow;

use ansi_term::Colour::{Green, Red, Yellow};
use clap::{App, Arg};
use structopt::StructOpt;

#[derive(Debug, Clone)]
struct Match {
    path: String,
    file: String,
    lines: Vec<(String, String)>,
}

pub trait MatchExt {
    fn file_path(&self) -> String;
}

impl MatchExt for &Match {
    fn file_path(&self) -> String {
        return format!("{}{}{}", self.path, std::path::MAIN_SEPARATOR, self.file);
    }
}

pub trait VecMatchExt {
    fn print(self);
}

impl VecMatchExt for Vec<Match> {
    fn print(self) {
        if self.len() > 0 {
            let status = format!("{} files with results found!", self.len());
            println!("{}", Green.paint(status));
            println!("");
            for m in &self {
                println!("");
                println!("> {}", Yellow.paint(m.path.clone()));
                println!("");
                let inner_status = format!("{} results in file found!", m.lines.len());
                println!("{}", Green.paint(inner_status));
                println!("");
                for l in &m.lines {
                    println!("{}  {}", l.0, l.1);
                }
            }
        } else {
            let status = format!("{} results found!", self.len());
            println!("{}", Red.paint(status));
        }
    }
}

fn main() {
    let args = App::new("fas")
                .version("0.0.1")
                .about("fas stands for find all stuff. With this app you can easily look for a string in your files names and inside your files.")
                .author("m4jrt0m")
                .args(&[
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .help("file to look into")
                        .takes_value(true),
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .help("Path to look into")
                        .takes_value(true),
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search term to look for")
                        .takes_value(true),
                ]).get_matches();

    println!("");
    println!("#########################################");
    println!("              ___");
    println!("             /  _| __ _____ ");
    println!("             | |_/  _` / __|");
    println!("             |  _| (_| \\__ \\");
    println!("             |_| \\___,_|___/");
    println!("");
    println!("              Find all stuff");
    println!("#########################################");
    println!("");

    let file: String = args.value_of_t("file").unwrap_or("".to_string());
    let path: String = args.value_of_t("path").unwrap_or(".".to_string());
    let search: String = args.value_of_t("search").unwrap_or("".to_string());

    let option_matches = get_files_with_matches(path, file, search);
    if option_matches.is_none() {
        println!("{}", Red.paint("No file or dir was given, leaving..."));
        return;
    }

    let matches = option_matches.unwrap();

    matches.print();
}

fn get_files_with_matches(path: String, file: String, search: String) -> Option<Vec<Match>> {
    if !path.is_empty() & file.is_empty() {
        let mut matches: Vec<Match> = Vec::new();
        matches.append(&mut search_folders_for_matches(path, search, Vec::new()));
        return Some(matches);;
    }

    if !path.is_empty() & !file.is_empty() {
        let mut matches: Vec<Match> = Vec::new();
        let file_match = get_file_lines_with_match(path, file, search);
        if file_match.is_some() {
            matches.push(file_match.unwrap());
        }
        return Some(matches);
    }
    
    // Error :: file or dir not given
    return None;
}

fn search_folders_for_matches(path: String, search: String, owner_matches: Vec<Match>) -> Vec<Match> {
    let mut results = owner_matches.to_vec();
    let folder_content = std::fs::read_dir(path).expect("unable to read dir");
    folder_content.for_each(|entry| {
                    if entry.as_ref().unwrap().metadata().unwrap().is_dir() {
                        results.append(&mut search_folders_for_matches(
                                entry.as_ref().unwrap().path().to_str().unwrap().to_string(),
                                search.clone(), 
                                owner_matches.clone()
                            ));
                    } else {
                        let file_match = get_file_lines_with_match(
                                entry.as_ref().unwrap().path().to_str().unwrap().to_string(),
                                entry.as_ref().unwrap().file_name().to_str().unwrap().to_string(),
                                search.clone());
                        if file_match.is_some() {
                            results.push(file_match.unwrap());
                        }
                    }
            
    });
    return results;
}

fn get_file_lines_with_match(path: String, file: String, search: String) -> Option<Match> {
    let file_content = std::fs::read_to_string(path.clone());
    if file_content.is_ok() {
        let elem_found = Match {path, file, lines: get_file_lines(file_content.unwrap(), search)};
    
        if elem_found.lines.len() != 0 {
            return Some(elem_found);
        }
    } else {
        //println!("Could not read {}", path);
    }

    return None;
}

fn get_file_lines(file_content: String, search: String) -> Vec<(String, String)> {
    let mut lines: Vec<(String, String)> = Vec::new();
    file_content.lines().enumerate().for_each(|(i, e)| {
        if e.contains(&search) {
            lines.push((
                i.to_owned().to_string(),
                e.to_string()
                    .replace(&search, Red.paint(search.as_str()).to_string().as_str()),
            ));
        }
    });
    return lines;
}
