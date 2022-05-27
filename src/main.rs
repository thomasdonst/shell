use shell::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(".");
    println!("{:?}", lexer.get_all_tokens())
}

