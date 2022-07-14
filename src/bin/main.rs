#![allow(warnings)]

use std::env;
use std::io::{Read, stdin, stdout, Write};
use shell::ast::Expr;
use shell::interpreter::Interpreter;
use shell::lexer::Lexer;
use shell::parser::Parser;
use shell::config::FOLDER_NAME;


fn main() {
    let program_dir = get_program_dir();
    let mut interpreter = Interpreter::new(&program_dir);
    loop {
        display_prompt();
        let input = read_input();
        let ast = parse_input(&input, &program_dir);
        match &ast {
            Ok(expr) => interpreter.eval(expr),
            Err(err) => eprintln!("{}", err)
        };
    }
}

fn get_program_dir() -> String {
    let prefix = env::current_dir().unwrap().display().to_string();
    prefix + FOLDER_NAME
}

fn display_prompt() {
    let cwd = env::current_dir().unwrap().display().to_string();
    print!("{}> ", cwd);
    stdout().flush().expect("Could not flush stdout")
}

fn read_input() -> String {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    input.trim().to_string()
}

fn parse_input(input: &str, program_dir: &str) -> Result<Expr, String> {
    let lexer = Lexer::new(&input, program_dir);
    let mut parser = Parser::new(lexer);
    parser.parse()
}