mod token;

use std::io::{self, Read};

use clap::error;
use thiserror::Error;
use token::Token;

// TODO: change this to using Iterator instead of Bytes<Read>
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
            '/' if matches!(self.input.peek(), Some(Ok(b)) if *b as char == '/') =>
            {
                let mut comment = String::new();
                self.input.next();

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
