use shell::lexer::Lexer;
use shell::token::Token;
use shell::utils::get_program_dir;

// NOTE: cat, grep, seq and echo should exist in the programs directory

fn get_tokens(input: &str) -> Vec<Token> {
    let program_dir = get_program_dir();
    let mut lexer = Lexer::new(input, &program_dir);
    lexer.get_tokens()
}

#[test]
fn lex_symbols_test() {
    let expected_tokens = vec![
        Token::OutputRedirect("".to_string()),
        Token::InputRedirect("".to_string()),
        Token::OutputRedirect("output".to_string()),
        Token::InputRedirect("input".to_string()),
        Token::Ampersand,
        Token::DoubleAmpersand,
        Token::Pipe,
        Token::Semicolon,
    ];
    assert_eq!(get_tokens("> < >output < input & && | ;"), expected_tokens);
    assert_eq!(get_tokens("><>output<input& &&|;"), expected_tokens);
}

#[test]
fn lex_arguments_test() {
    let expected_tokens = vec![
        Token::String("only".to_string()),
        Token::String("arguments".to_string()),
    ];
    assert_eq!(get_tokens("Only arguments"), expected_tokens);
}

#[test]
fn lex_hyphens_test() {
    let expected_tokens = vec![
        Token::Hyphen("-a".to_string()),
        Token::Hyphen("-abc".to_string()),
        Token::Hyphen("-".to_string()),
    ];
    assert_eq!(get_tokens("-a -abc-"), expected_tokens);
}

#[test]
fn lex_double_hyphens_test() {
    let expected_tokens = vec![
        Token::DoubleHyphen("--option".to_string()),
        Token::DoubleHyphen("--".to_string()),
        Token::DoubleHyphen("--option2".to_string()),
    ];
    assert_eq!(get_tokens("--option-- --option2"), expected_tokens);
}

#[test]
fn lex_builtin_commands_test() {
    let expected_tokens = vec![
        Token::Command("cd".to_string()),
        Token::Command("clear".to_string()),
    ];
    assert_eq!(get_tokens("Cd Clear"), expected_tokens);
}

#[test]
fn lex_complex_command_test() {
    let expected_tokens = vec![
        Token::Command("cat".to_string()),
        Token::String("a.txt".to_string()),
        Token::Pipe,
        Token::Command("grep".to_string()),
        Token::String("h".to_string()),
        Token::DoubleAmpersand,
        Token::Command("seq".to_string()),
        Token::String("3".to_string()),
        Token::Semicolon,
        Token::Command("echo".to_string()),
        Token::String("hello".to_string()),
    ];
    assert_eq!(get_tokens("cat a.txt | grep h &&seq 3 ; echo hello"), expected_tokens);
}