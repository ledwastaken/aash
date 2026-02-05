use std::io::Read;

pub mod token;

pub use self::token::Token;

pub struct Lexer<R: Read> {
    src: R,
    current: Option<Token>,
}

impl<R: Read> Lexer<R> {
    pub fn new(src: R) -> Self {
        Lexer { src, current: None }
    }

    pub fn peek(&mut self) -> &Token {
        if self.current.is_none() {
            self.process_next_token();
        }

        self.current.as_ref().unwrap()
    }

    pub fn pop(&mut self) {
        self.current = None;
    }

    fn process_next_token(&mut self) {
        self.current = Some(Token::Eof); // FIXME
    }
}
