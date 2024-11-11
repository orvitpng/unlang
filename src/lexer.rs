use std::io::{self, Read};

use thiserror::Error;

// TODO: change this to using Iterator instead of Bytes<Read>
pub struct Lexer<R: Read> {
    input: io::Bytes<R>,
}
impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Lexer {
            input: reader.bytes(),
        }
    }
}
impl<R: Read> Iterator for Lexer<R> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Error, Debug)]
pub enum LexerError {}

#[derive(Debug, PartialEq)]
pub enum Token {
}
