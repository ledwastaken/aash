use std::io::Read;

use crate::ast::Ast;
use crate::lexer::{Lexer, Token};

enum ParseResult {
    Success(Ast),
    UnexpectedToken(Token),
}

pub fn parse_input<R: Read>(lexer: &mut Lexer<R>) -> Option<Ast> {
    let token = lexer.next();

    match token {
        Token::Eof => None,
        Token::Newline => Some(Ast::None),
        token => match parse_list(lexer, token) {
            ParseResult::Success(ast) => Some(ast),
            ParseResult::UnexpectedToken(_) => Some(Ast::None) // TODO handle parse error
        },
    }
}

fn parse_list<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    ParseResult::Success(Ast::None) // TODO
}
