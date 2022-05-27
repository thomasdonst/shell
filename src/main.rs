use shell::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("comannd << delimiter");
    println!("{:?}", lexer.get_all_tokens())
}

