use std::io::{Bytes, Read};

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    pub process_reserved_words: bool,

    stream: Bytes<R>,
    next_token: Option<Token>,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        let stream = reader.bytes();

        Lexer {
            process_reserved_words: true,
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
                // TODO: add an inner enum to Token::Word to differ single/double quote
                '\'' => {
                    enum Stop {
                        FoundQuote(Vec<u8>),
                        Error,
                    }

                    let bytes = self.stream.try_fold(Vec::new(), |mut acc, val| match val {
                        Ok(b'\'') => Err(Stop::FoundQuote(acc)),
                        Ok(byte) => {
                            acc.push(byte);
                            Ok(acc)
                        }
                        Err(_) => Err(Stop::Error),
                    });

                    return match bytes {
                        Ok(_) => {
                            eprintln!("unexpected EOF while looking for matching `''");
                            Token::Eof
                        }
                        Err(Stop::FoundQuote(v)) => {
                            Token::Word(String::from_utf8_lossy(&v).into_owned())
                        }
                        Err(Stop::Error) => Token::Eof,
                    };
                }
                '#' => {
                    while let Some(Ok(ch)) = self.stream.next() {
                        if ch == b'\n' {
                            break;
                        }
                    }

                    return Token::Newline;
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
        if self.process_reserved_words {
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
