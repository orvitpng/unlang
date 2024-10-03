use std::io::{self, Read};

pub struct Lexer<R: Read> {
    input: io::Bytes<R>,
    eof_read: bool,
}
impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Lexer {
            input: reader.bytes(),
            eof_read: false,
        }
    }
}
impl<R: Read> Iterator for Lexer<R> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_read {
            return None;
        }

        match self.input.next() {
            Some(Ok(char)) => Some(Err(LexerError::InvalidCharacter(char as char))),
            Some(Err(e)) => Some(Err(LexerError::IoError(e))),
            None => {
                self.eof_read = true;
                Some(Ok(Token::Eof))
            }
        }
    }
}

#[derive(Debug)]
// todo: remove this. just telling the compiler to shut up for now
#[allow(dead_code)]
pub enum LexerError {
    InvalidCharacter(char),
    IoError(io::Error),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
}
