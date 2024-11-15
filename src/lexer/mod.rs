mod token;

use std::io::{self, Read};

use thiserror::Error;
use token::Token;

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
        let c = match self.input.next()? {
            Ok(b) => b as char,
            Err(err) => return Some(Err(LexerError::IoError(err))),
        };

        if c.is_whitespace() {
            return self.next();
        }

        Some(Ok(match c {
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::from(c);

                while let Some(Ok(next_b)) = self.input.peek() {
                    let next_c = *next_b as char;
                    if !next_c.is_alphanumeric() && next_c != '_' {
                        break;
                    }

                    ident.push(next_c);
                    self.input.next();
                }

                Token::parse_keydentifier(ident)
            }
            '_' => Token::Identifier(token::Identifier::Discard),

            '/' if matches!(self.input.peek(), Some(Ok(b)) if *b as char == '/') =>
            {
                let mut comment = String::new();
                self.input.next();

                // hopefully not a problem for error handling
                while let Some(Ok(b)) = self.input.next() {
                    if b as char == '\n' {
                        break;
                    }
                    comment.push(b as char);
                }

                Token::Comment(comment)
            }

            _ => return Some(Err(LexerError::UnexpectedChar(c))),
        }))
    }
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("unexpected character `{0}`")]
    UnexpectedChar(char),
}
