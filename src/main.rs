use std::process::{Command, Stdio};
use shell::interpreter::Interpreter;
use shell::lexer::Lexer;
use shell::parser::Parser;

fn main() {
    let mut lexer = Lexer::new("cat | pwd");
    // println!("{:?}", lexer.get_all_tokens());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    match &ast {
        Ok(expr) => {
            let mut interpreter = Interpreter::new();
            println!("{}", interpreter.eval(expr));
        }
        Err(s) => println!("{:?}", s)
    };
}