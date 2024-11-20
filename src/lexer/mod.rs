mod macros;
mod token;

use std::io::{self, Read};

use macros::iter_matcher;
use thiserror::Error;
use token::{Delimiter, Operator, Token};

pub struct Lexer<R: Read> {
    input: std::iter::Peekable<io::Bytes<R>>,
}
impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Lexer {
            input: reader.bytes().peekable(),
        }
    }
}
impl<R: Read> Iterator for Lexer<R> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let input = &mut self.input;

        let c = match input.next()? {
            Ok(b) => b as char,
            Err(err) => return Some(Err(LexerError::IoError(err))),
        };

        if c.is_whitespace() {
            return self.next();
        }

        Some(Ok(iter_matcher!(input, c, {
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::from(c);

                while let Some(Ok(next_b)) = input.peek() {
                    let next_c = *next_b as char;
                    if !next_c.is_alphanumeric() && next_c != '_' {
                        break;
                    }

                    ident.push(next_c);
                    input.next();
                }

                Token::parse_keydentifier(ident)
            },

            '=' => Token::Operator(Operator::Assignment),
            ':' => Token::Operator(Operator::Type),
            '-', '>' => Token::Operator(Operator::ReturnType),

            '(' => Token::Delimiter(Delimiter::OpenParen),
            ')' => Token::Delimiter(Delimiter::CloseParen),
            '{' => Token::Delimiter(Delimiter::OpenBrace),
            '}' => Token::Delimiter(Delimiter::CloseBrace),

            ',' => Token::Separator,
            '/', '/' => {
                let mut comment = String::new();

                // hopefully not a problem for error handling
                while let Some(Ok(b)) = input.next() {
                    if b == b'\n' {
                        break;
                    }
                    comment.push(b as char);
                }

                Token::Comment(comment)
            },

            _ => return Some(Err(LexerError::UnexpectedChar(c))),
        })))
    }
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("unexpected character `{0}`")]
    UnexpectedChar(char),
}
