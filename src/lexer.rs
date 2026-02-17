use std::io::{Bytes, Read};

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    stream: Bytes<R>,
    next_token: Option<Token>,
    process_command_start: bool,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        let stream = reader.bytes();

        Lexer {
            stream,
            next_token: None,
            process_command_start: false,
        }
    }

    pub fn next(&mut self) -> Token {
        if let Some(token) = self.next_token.take() {
            token
        } else {
            self.process_next_token()
        }
    }

    pub fn set_command_start(&mut self) {
        self.process_command_start = true;
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
                        return self.process_word(token);
                    }
                }
                ';' => {
                    if token.is_empty() {
                        return Token::Semicolon;
                    } else {
                        self.next_token = Some(Token::Semicolon);
                        return self.process_word(token);
                    }
                }
                ch if ch.is_whitespace() => {
                    if !token.is_empty() {
                        return self.process_word(token);
                    }
                }
                ch => token.push(ch),
            }
        }

        if !token.is_empty() {
            self.process_word(token)
        } else {
            Token::Eof
        }
    }

    fn process_word(&mut self, word: String) -> Token {
        if self.process_command_start {
            self.process_command_start = false;
            match word.as_str() {
                "if" => Token::If,
                "then" => Token::Then,
                "elif" => Token::Elif,
                "else" => Token::Else,
                "fi" => Token::Fi,
                _ => Token::Word(word),
            }
        } else {
            Token::Word(word)
        }
    }
}
