mod token;

use std::io::{self, Bytes, Read};

use thiserror::Error;
use token::{Delimiter, Operator, Token};

macro_rules! iter_matcher {
    ($input:expr, $c:expr =>
        $($cust_ident:ident if $cust_if:expr => $cust_expr:expr,)*
        $($lit1_literal:literal $(, $lit_literal:literal)* => $lit_expr:expr,)*
        _ => $unhandled:expr $(,)?
    ) => {
        match $c {
            $($cust_ident if $cust_if => $cust_expr,)*
            $(
                $lit1_literal
                if consume_if_eq($input, &[$($lit_literal,)*])
                => $ lit_expr,)*
            _ => $unhandled,
        }
    };
}

pub struct Lexer<R: Read> {
    input: MultiPeek<Bytes<R>>,
}
impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Lexer {
            input: MultiPeek::new(reader.bytes()),
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

        Some(Ok(iter_matcher! { input, c =>
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

struct MultiPeek<I: Iterator> {
    iter: I,
    buf: Vec<I::Item>,
}
impl<I: Iterator> MultiPeek<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            buf: Vec::new(),
        }
    }

    fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        while self.buf.len() <= n {
            if let Some(i) = self.iter.next() {
                self.buf.push(i);
            } else {
                return None;
            }
        }
        Some(&self.buf[n])
    }

    fn skip(&mut self, n: usize) {
        let buf_n = self.buf.len().min(n);
        self.buf.drain(0..buf_n);

        let r = n - buf_n;
        if r != 0 {
            self.iter.nth(r - 1);
        }
    }

    fn next(&mut self) -> Option<I::Item> {
        if !self.buf.is_empty() {
            Some(self.buf.remove(0))
        } else {
            self.iter.next()
        }
    }
}

fn consume_if_eq<R: Read>(
    iter: &mut MultiPeek<Bytes<R>>,
    chars: &[char],
) -> bool {
    if chars.is_empty() {
        return true;
    }

    for (i, &c) in chars.iter().enumerate() {
        match iter.peek_nth(i) {
            Some(Ok(b)) => {
                if *b as char != c {
                    return false;
                }
            }
            _ => return false,
        }
    }

    iter.skip(chars.len());
    true
}
