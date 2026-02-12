use std::io::{self, Bytes, Read};
use std::iter::Peekable;

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    stream: Peekable<Bytes<R>>,
    current: Token,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> io::Result<Self> {
        let mut stream = reader.bytes().peekable();
        let token = process_next_token(&mut stream)?;

        Ok(Lexer {
            stream,
            current: token,
        })
    }

    pub fn peek(&self) -> &Token {
        &self.current
    }

    pub fn pop(&mut self) -> io::Result<()> {
        self.current = process_next_token(&mut self.stream)?;
        Ok(())
    }
}

fn process_next_token<R: Read>(bytes: &mut Peekable<Bytes<R>>) -> io::Result<Token> {
    let mut token = String::new();
    // let mut single_quote = false;
    // let mut double_quote = false;

    while let Some(Ok(value)) = bytes.peek() {
        if *value as char == '\n' {
            return Ok(Token::Newline);
        } else if (*value as char).is_whitespace() {
            if !token.is_empty() {
                return Ok(Token::Word(token));
            }
        } else {
            token.push(*value as char);
        }

        bytes.next();
    }

    if !token.is_empty() {
        Ok(Token::Word(token))
    } else {
        Ok(Token::Eof)
    }
}
