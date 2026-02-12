use std::io::{self, Bytes, Read};

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    stream: Bytes<R>,
    current: Token,
}

impl<R: Read> Lexer<R> {
    pub fn new(mut stream: Bytes<R>) -> io::Result<Self> {
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

fn process_next_token<R: Read>(bytes: &mut Bytes<R>) -> io::Result<Token> {
    let mut token = String::new();
    // let mut single_quote = false;
    // let mut double_quote = false;

    while let Some(Ok(value)) = bytes.next() {
        if value as char == '\n' {
            return Ok(Token::Newline);
        } else if (value as char).is_whitespace() {
            if !token.is_empty() {
                return Ok(Token::Word(token));
            }
        } else {
            token.push(value as char);
        }
    }

    if !token.is_empty() {
        Ok(Token::Word(token))
    } else {
        Ok(Token::Eof)
    }
}
