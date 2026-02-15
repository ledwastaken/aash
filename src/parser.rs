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
            ParseResult::UnexpectedToken(_) => Some(Ast::None), // TODO handle parse error
        },
    }
}

fn parse_list<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_and_or(lexer, token)
}

fn parse_and_or<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_pipeline(lexer, token)
}

fn parse_pipeline<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_command(lexer, token)
}

fn parse_command<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_simple_command(lexer, token)
}

fn parse_simple_command<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    match token {
        Token::Word(word) => {
            let mut words = vec![word];
            let mut token = lexer.next();

            while let Token::Word(next_word) = token {
                words.push(next_word);
                token = lexer.next();
            }

            ParseResult::Success(Ast::SimpleCommand { words })
        }
        token => ParseResult::UnexpectedToken(token),
    }
}
