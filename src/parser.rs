use std::io::Read;

use crate::ast::Ast;
use crate::lexer::{Lexer, Token};

pub fn parse_input<R: Read>(lexer: &mut Lexer<R>) -> Option<Ast> {
    match lexer.peek() {
        Token::Eof => {
            lexer.pop();
            None
        }
        Token::Newline => {
            lexer.pop();
            Some(Ast::None)
        }
        _ => {
            // TODO parse_list
            lexer.pop();
            Some(Ast::None)
        }
    }
}
