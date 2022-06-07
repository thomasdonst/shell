use shell::interpreter::{Interpreter};
use shell::lexer::Lexer;
use shell::parser::{Parser};

fn main() {
    let lexer = Lexer::new("ls");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    println!("{}", interpreter.eval(&ast));
}