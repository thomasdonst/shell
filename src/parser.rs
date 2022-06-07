use std::io::{Error, ErrorKind};
use std::rc::Rc;
use std::iter::Peekable;
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Cmd {
    Cat,
    Pwd,
    Cd,
    Ls,
    Cp,
    Mv,
    // Mkdir,
    // Rmdir,
    // Rm,
    // Touch,
    // Locate,
    // Find,
    // Grep,
    // Kill,
    // Ping,
    // History,
    // Man,
    // Echo,
    // Sort,
}

// #[derive(Debug)]
// pub enum IOModifier {
//     Great {
//         filename: Rc<Expr>
//     },
//     DoubleGreat {
//         filename: Rc<Expr>
//     },
//     GreatAmpersand {
//         filename: Rc<Expr>
//     },
//     Less {
//         filename: Rc<Expr>
//     },
//     DoubleLess {
//         filename: Rc<Expr>
//     },
// }

#[derive(Debug)]
pub enum Expr {
    Cmd {
        ty: Rc<Cmd>,
        options: Vec<char>,
        arguments: Vec<String>,
    },
    Pipe {
        left: Rc<Expr>,
        right: Rc<Expr>,
    },
    // IOModifier {
    //     modifier: Rc<IOModifier>
    // },
}

pub struct Parser<'lexer> {
    lexer: Peekable<Lexer<'lexer>>,
    current: Option<Token>,
}

impl<'lexer> Parser<'lexer> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
            current: None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.current = self.lexer.next();
        self.current.clone()
    }

    fn peek(&mut self) -> Option<Token> {
        self.lexer.peek().cloned()
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.parse_binary()
    }

    fn parse_binary(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.parse_command()?;
        loop {
            if !self.expect_token(Token::Pipe) {
                break;
            }
            self.next();
            let rhs = self.parse_binary()?;
            lhs = Expr::Pipe {
                left: Rc::new(lhs),
                right: Rc::new(rhs),
            }
        }
        Ok(lhs)
    }

    fn parse_command(&mut self) -> Result<Expr, Error> {
        match self.next() {
            Some(Token::Command(Cmd::Cat)) => self.parse_cat(),
            Some(Token::Command(Cmd::Pwd)) => self.parse_pwd(),
            Some(Token::Command(Cmd::Cd)) => self.parse_cd(),
            Some(Token::Command(Cmd::Ls)) => self.parse_ls(),
            Some(Token::Command(Cmd::Cp)) => self.parse_cp(),
            Some(Token::Command(Cmd::Mv)) => self.parse_mv(),
            Some(x) => Err(Error::new(ErrorKind::Other, x.to_string() + " is not a command token")),
            None => Err(Error::new(ErrorKind::InvalidInput, "Expected a token")),
        }
    }

    fn parse_cat(&mut self) -> Result<Expr, Error> {
        let mut options = Vec::new();
        while let Some(Token::Option(s)) = self.peek() {
            let tmp = s.chars().collect();
            options = [options, tmp].concat();
            self.next();
        }

        let mut arguments = Vec::new();
        while let Some(Token::Argument(s)) = self.peek() {
            arguments.push(s);
            self.next();
        }

        Ok(
            Expr::Cmd {
                ty: Rc::new(Cmd::Cat),
                options,
                arguments,
            }
        )
    }

    fn parse_pwd(&mut self) -> Result<Expr, Error> {
        unimplemented!()
    }

    fn parse_cd(&mut self) -> Result<Expr, Error> {
        unimplemented!()
    }

    fn parse_ls(&mut self) -> Result<Expr, Error> {
        unimplemented!()
    }

    fn parse_cp(&mut self) -> Result<Expr, Error> {
        unimplemented!()
    }

    fn parse_mv(&mut self) -> Result<Expr, Error> {
        unimplemented!()
    }

    fn expect_token(&mut self, should: Token) -> bool {
        match self.peek() {
            Some(Token::Option(s)) => {
                should == Token::Option(s)
            }
            Some(Token::Argument(s)) => {
                should == Token::Argument(s)
            }
            Some(Token::EnvVariable(s)) => {
                should == Token::EnvVariable(s)
            }
            Some(is) => {
                is == should
            }
            _ => false
        }
    }

// fn expect_tokens(&mut self, tokens: Vec<Token>) -> Result<Expr, String> {
//     for t in tokens {
//         let result = self.expect_token(t);
//         if result.is_err() {
//             return result;
//         }
//         self.next();
//     }
//     Ok(Expr::Word(String::from("")))
// }
}