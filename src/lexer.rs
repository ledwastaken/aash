use std::io::{Bytes, Error, Read};

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    src: Bytes<R>,
    current: Token,
}

impl<R: Read> Lexer<R> {
    pub fn new(src: R) -> Result<Self, Error> {
        let mut bytes = src.bytes();
        let token = process_next_token(&mut bytes)?;

        Ok(Lexer {
            src: bytes,
            current: token,
        })
    }

    pub fn peek(&mut self) -> &Token {
        &self.current
    }

    pub fn pop(&mut self) -> Result<(), Error> {
        self.current = process_next_token(&mut self.src)?;
        Ok(())
    }
}

fn process_next_token<R: Read>(bytes: &mut Bytes<R>) -> Result<Token, Error> {
    let mut token = String::new();
    let mut single_quote = false;
    let mut double_quote = false;

    while let Some(value) = bytes.next() {
        // FIXME
    }

    Ok(Token::Eof)
}
