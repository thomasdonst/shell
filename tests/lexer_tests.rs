#![allow(warnings)]

use shell::config::FOLDER_NAME;
use shell::lexer::Lexer;
use shell::token::Token;

fn get_tokens(input: &str) -> Vec<Token>{
    let mut lexer = Lexer::new(input, FOLDER_NAME);
    lexer.get_tokens()
}

#[test]
fn lex_symbols_test() {
    let input = "> >>   >& <  << & && | = \" ;";
    let expected_tokens = vec![
        Token::Great,
        Token::DoubleGreat,
        Token::GreatAmpersand,
        Token::Less,
        Token::DoubleLess,
        Token::Ampersand,
        Token::DoubleAmpersand,
        Token::Pipe,
        Token::Equal,
        Token::Quote,
        Token::Semicolon,
    ];
    assert_eq!(get_tokens(input), expected_tokens);
}

#[test]
fn lex_commands_test() {
    let input = "Cd Clear";
    let expected_tokens = vec![
        Token::Command("cd".to_string()),
        Token::Command("clear".to_string()),
    ];
    assert_eq!(get_tokens(input), expected_tokens);
}

#[test]
fn lex_arguments_test() {
    let input = "Only arguments";
    let expected_tokens = vec![
        Token::Argument("only".to_string()),
        Token::Argument("arguments".to_string()),
    ];
    assert_eq!(get_tokens(input), expected_tokens);
}

#[test]
fn lex_hyphens_test() {}

#[test]
fn lex_double_hyphens_test() {}

#[test]
fn lex_env_variables_test() {}