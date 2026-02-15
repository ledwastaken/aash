use std::io::{Bytes, Read};
use std::iter::Peekable;

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    stream: Peekable<Bytes<R>>,
    next_token: Option<Token>,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        let stream = reader.bytes().peekable();

        Lexer {
            stream,
            next_token: None,
        }
    }

    pub fn next(&mut self) -> Token {
        if let Some(token) = self.next_token.take() {
            token
        } else {
            process_next_token(&mut self.stream)
        }
    }
}

fn process_next_token<R: Read>(bytes: &mut Peekable<Bytes<R>>) -> Token {
    let mut token = String::new();

    while let Some(Ok(value)) = bytes.peek() {
        if *value as char == '\n' {
            if token.is_empty() {
                bytes.next();
                return Token::Newline;
            } else {
                return Token::Word(token);
            }
        } else if (*value as char).is_whitespace() {
            if !token.is_empty() {
                return Token::Word(token);
            }
        } else {
            token.push(*value as char);
        }

        bytes.next();
    }

    if !token.is_empty() {
        Token::Word(token)
    } else {
        Token::Eof
    }
}
