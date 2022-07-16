use shell::config::FOLDER_NAME;
use shell::lexer::Lexer;
use shell::token::Token;
use shell::utils::get_program_dir;

// NOTE: cat, grep, seq and echo should exist in the programs directory in order to pass the tests

fn get_tokens(input: &str) -> Vec<Token> {
    let program_dir = get_program_dir();
    let mut lexer = Lexer::new(input, &program_dir);
    lexer.get_tokens()
}

#[test]
fn lex_symbols_test() {
    let expected_tokens = vec![
        Token::Great,
        Token::Less,
        Token::Ampersand,
        Token::DoubleAmpersand,
        Token::Pipe,
        Token::Equal,
        Token::Quote,
        Token::Semicolon,
    ];
    assert_eq!(get_tokens("> < & && | = \" ;"), expected_tokens);
    assert_eq!(get_tokens("><& &&|=\";"), expected_tokens);
}

#[test]
fn lex_arguments_test() {
    let expected_tokens = vec![
        Token::Argument("only".to_string()),
        Token::Argument("arguments".to_string()),
    ];
    assert_eq!(get_tokens("Only arguments"), expected_tokens);
}

#[test]
fn lex_hyphens_test() {
    let expected_tokens = vec![
        Token::Hyphen("a".to_string()),
        Token::Hyphen("abc".to_string()),
        Token::Hyphen("".to_string()),
    ];
    assert_eq!(get_tokens("-a -abc-"), expected_tokens);
}

#[test]
fn lex_double_hyphens_test() {
    let expected_tokens = vec![
        Token::DoubleHyphen("option".to_string()),
        Token::DoubleHyphen("".to_string()),
        Token::DoubleHyphen("option2".to_string()),
    ];
    assert_eq!(get_tokens("--option-- --option2"), expected_tokens);
}

#[test]
fn lex_env_variables_test() {
    let expected_tokens = vec![
        Token::EnvVariable("var".to_string()),
        Token::EnvVariable("".to_string()),
        Token::EnvVariable("".to_string()),
        Token::EnvVariable("var2".to_string()),
    ];
    assert_eq!(get_tokens("$var$$$var2"), expected_tokens);
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
        Token::Argument("a.txt".to_string()),
        Token::Pipe,
        Token::Command("grep".to_string()),
        Token::Argument("h".to_string()),
        Token::DoubleAmpersand,
        Token::Command("seq".to_string()),
        Token::Argument("3".to_string()),
        Token::Semicolon,
        Token::Command("echo".to_string()),
        Token::EnvVariable("hello".to_string()),
    ];
    assert_eq!(get_tokens("cat a.txt | grep h &&seq 3 ; echo $hello"), expected_tokens);
}