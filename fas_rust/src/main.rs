#![allow(unused)]

use structopt::StructOpt;
use clap::{Arg,App};
use ansi_term::Colour::{Red,Green};

fn main() {
    let args = App::new("fas")
                .version("0.0.1")
                .about("fas stands for find all stuff. With this app you can easily look for a string in your files names and inside your files.")
                .author("m4jrt0m")
                .args(&[
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
    println!("              __");
    println!("             / _| __ _ ___ ");
    println!("             | |_/  _` / __|");
    println!("             |  _| (_| \\__ \\");
    println!("             |_| \\___,_|___/");
    println!("");
    println!("#########################################");
    println!("");


    let path: String = args.value_of_t("path").unwrap_or(".".to_string());
    let search: String = args.value_of_t("search").unwrap_or("".to_string());
    let pwd = path.clone();

    let content = std::fs::read_to_string(path)
        .expect("could not read file");

    let lines = get_file_lines(content, search);

    if lines.len() > 0 {
        let status = format!("{} results found!", lines.len());
        println!("{}", Green.paint(status));
        println!("");
        println!("> {}", pwd);
        println!("");
        for l in &lines {
            println!("{}  {}", l.0, l.1);
        }
    } else {
        let status = format!("{} results found!", lines.len());
        println!("{}", Red.paint(status));
    }
}

fn get_file_lines(content: String, search: String) -> Vec<(String, String)> {
    let mut lines: Vec<(String, String)> = Vec::new();  
    content.lines().enumerate().for_each(|(i, e)| {
        if e.contains(&search) {
            lines.push((
                        i.to_owned().to_string(), 
                        e.to_string().replace(&search, Red.paint(search.as_str()).to_string().as_str()
                        )));
        }
    }); 
    return lines;
}
