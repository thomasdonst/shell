use std::fs::File;
use std::{env, io};
use std::io::Read;
use std::path::Path;
use crate::ast::Expr;
use crate::config::FOLDER_NAME;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn get_program_dir() -> String {
    let prefix = env::current_dir().unwrap().display().to_string();
    prefix + FOLDER_NAME
}

pub fn parse(input: &str, program_dir: &str) -> Result<Expr, String> {
    let lexer = Lexer::new(&input, program_dir);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

pub fn read_stdin() -> String {
    let mut result = String::new();
    let lines = io::stdin().lines();
    lines.for_each(|l| result.push_str(&*format!("{}\r\n", l.unwrap())));
    result[0..result.len() - 1].to_string()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn read_file(path: &str) -> Result<String, String> {
    match File::open("./".to_string() + path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                Ok(contents)
            } else {
                Err(format!("Can't read file: {}", path))
            }
        }
        Err(_) => Err(format!("Can't open file: {}", path))
    }
}

pub fn read_files(files: Vec<String>, stdout: &mut String, stderr: &mut String) {
    for file in &files {
        match read_file(&file) {
            Ok(result) => {
                stdout.push_str(&(result + "\r\n"));
            }
            Err(error) => {
                stderr.push_str(&(error + "\r\n"));
            }
        }
    }
    if stdout.len() >= 2 {
        stdout.truncate(stdout.len() - 2)
    }
    if stderr.len() >= 2 {
        stderr.truncate(stderr.len() - 2)
    }
}

pub fn process_options(options: Vec<String>, pref: Vec<char>) -> Result<Vec<char>, char> {
    let mut valid_result: Vec<char> = Vec::new();
    for option in options {
        let chars = option.chars().collect::<Vec<char>>();
        for char in chars {
            match pref.contains(&char) {
                true => valid_result.push(char),
                false => return Err(char)
            }
        }
    }
    Ok(valid_result)
}

pub fn get_args() -> Vec<String> {
    env::args().skip(1).collect::<Vec<String>>()
}

pub fn eq(arguments: &Vec<String>) -> bool {
    let first_arg = arguments.first().unwrap();
    arguments
        .iter()
        .skip(1)
        .all(|arg| first_arg == arg)
}
