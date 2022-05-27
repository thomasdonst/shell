// #![allow(warnings)]
//
// use pretty_assertions::{assert_eq, assert_ne};
// use log::{info, trace, warn};
//
// use shell::lexer::Lexer;
// use shell::token::Token;
//
// #[test]
// fn lex_symbols_test() {
//     let input = ">>   >& <  << & |";
//     let mut lexer = Lexer::new(input);
//
//     let expected_tokens = vec![
//         Token::DoubleGreat,
//         Token::Whitespace,
//         Token::GreatAmpersand,
//         Token::Whitespace,
//         Token::Less,
//         Token::Whitespace,
//         Token::DoubleLess,
//         Token::Whitespace,
//         Token::Ampersand,
//         Token::Whitespace,
//         Token::Pipe,
//     ];
//
//     for token in expected_tokens {
//         assert_eq!(lexer.get_all_tokens(), token);
//     }
// }
//
// #[test]
// fn lex_identifiers_test() {}