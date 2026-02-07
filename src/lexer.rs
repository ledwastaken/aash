use std::io::{Bytes, Read};

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    src: Bytes<R>,
    current: Token,
}

impl<R: Read> Lexer<R> {
    pub fn new(src: R) -> Self {
        let bytes = src.bytes();
        let token = process_next_token(&bytes);

        Lexer {
            src: bytes,
            current: token,
        }
    }

    pub fn peek(&mut self) -> &Token {
        &self.current
    }

    pub fn pop(&mut self) {
        self.current = process_next_token(&self.src);
    }
}

fn process_next_token<R: Read>(bytes: &Bytes<R>) -> Token {
    Token::Eof // FIXME
}
