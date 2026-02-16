use std::io::{Bytes, Read};

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    stream: Bytes<R>,
    next_token: Option<Token>,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        let stream = reader.bytes();

        Lexer {
            stream,
            next_token: None,
        }
    }

    pub fn next(&mut self) -> Token {
        if let Some(token) = self.next_token.take() {
            token
        } else {
            self.process_next_token()
        }
    }

    fn process_next_token(&mut self) -> Token {
        let mut token = String::new();

        // TODO: handle io::Error
        while let Some(Ok(value)) = self.stream.next() {
            match value as char {
                '\n' => {
                    if token.is_empty() {
                        return Token::Newline;
                    } else {
                        self.next_token = Some(Token::Newline);
                        return Token::Word(token);
                    }
                }
                ';' => {
                    if token.is_empty() {
                        return Token::Semicolon;
                    } else {
                        self.next_token = Some(Token::Semicolon);
                        return Token::Word(token);
                    }
                }
                ch if ch.is_whitespace() => {
                    if !token.is_empty() {
                        return Token::Word(token);
                    }
                }
                ch => token.push(ch),
            }
        }

        if !token.is_empty() {
            Token::Word(token)
        } else {
            Token::Eof
        }
    }
}
