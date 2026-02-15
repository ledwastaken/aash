use std::io::Read;

use crate::ast::Ast;
use crate::lexer::{Lexer, Token};

pub fn parse_input<R: Read>(lexer: &mut Lexer<R>) -> Option<Ast> {
    let token = lexer.next();

    println!("{:?}", token);

    match token {
        Token::Eof => None,
        Token::Newline => Some(Ast::None),
        _ => Some(Ast::None), // TODO parse_list
    }
}
