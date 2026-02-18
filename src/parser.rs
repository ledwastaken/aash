use std::io::Read;

use crate::ast::Ast;
use crate::lexer::{Lexer, Token};

enum ParseResult {
    Success(Ast, Token),
    UnexpectedToken(Token),
}

pub fn parse_input<R: Read>(lexer: &mut Lexer<R>) -> Option<Ast> {
    let token = lexer.next();

    match token {
        Token::Eof => None,
        Token::Newline => Some(Ast::None),
        token => match parse_list(lexer, token) {
            ParseResult::Success(ast, Token::Eof) => Some(ast),
            ParseResult::Success(ast, Token::Newline) => Some(ast),
            ParseResult::Success(_, unexpected_token) => {
                eprintln!("Unexpected token: {:?}", unexpected_token);
                Some(Ast::None)
            }
            ParseResult::UnexpectedToken(unexpected_token) => {
                eprintln!("Unexpected token: {:?}", unexpected_token);
                Some(Ast::None)
            }
        },
    }
}

fn parse_list<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    match parse_and_or(lexer, token) {
        ParseResult::Success(ast, Token::Semicolon) => {
            let mut commands = vec![ast];

            loop {
                let next_token = lexer.next();
                match parse_and_or(lexer, next_token) {
                    ParseResult::Success(next_ast, Token::Semicolon) => commands.push(next_ast),
                    ParseResult::Success(next_ast, last_token) => {
                        commands.push(next_ast);
                        return ParseResult::Success(Ast::List(commands), last_token);
                    }
                    ParseResult::UnexpectedToken(unexpected_token) => {
                        return ParseResult::Success(Ast::List(commands), unexpected_token);
                    }
                }
            }
        }
        e => e,
    }
}

fn parse_and_or<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_pipeline(lexer, token)
}

fn parse_pipeline<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_command(lexer, token)
}

fn parse_command<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    let result = parse_simple_command(lexer, token);

    if let ParseResult::UnexpectedToken(unexpected_token) = result {
        parse_shell_command(lexer, unexpected_token)
    } else {
        result
    }
}

fn parse_simple_command<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    match token {
        Token::Word(word) => {
            lexer.process_reserved_words = false;

            let mut words = Vec::new();
            let mut token = lexer.next();

            while let Token::Word(next_word) = token {
                words.push(next_word);
                token = lexer.next();
            }

            lexer.process_reserved_words = true;

            ParseResult::Success(
                Ast::SimpleCommand {
                    program: word,
                    args: words,
                },
                token,
            )
        }
        token => ParseResult::UnexpectedToken(token),
    }
}

fn parse_shell_command<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    parse_rule_if(lexer, token)
}

fn parse_rule_if<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    if token != Token::If {
        return ParseResult::UnexpectedToken(token);
    } else {
        let mut next_token = lexer.next();
        match parse_compound_list(lexer, next_token) {
            ParseResult::Success(condition, Token::Then) => {
                next_token = lexer.next();
                match parse_compound_list(lexer, next_token) {
                    ParseResult::Success(then_branch, Token::Fi) => ParseResult::Success(
                        Ast::IfCommand {
                            condition: Box::new(condition),
                            then_branch: Box::new(then_branch),
                            else_branch: None,
                        },
                        lexer.next(),
                    ),
                    ParseResult::Success(then_branch, else_token) => {
                        match parse_else_clause(lexer, else_token) {
                            ParseResult::Success(else_branch, Token::Fi) => ParseResult::Success(
                                Ast::IfCommand {
                                    condition: Box::new(condition),
                                    then_branch: Box::new(then_branch),
                                    else_branch: Some(Box::new(else_branch)),
                                },
                                lexer.next(),
                            ),
                            e => e,
                        }
                    }
                    e => e,
                }
            }
            ParseResult::Success(_, unexpected_token) => {
                ParseResult::UnexpectedToken(unexpected_token)
            }
            ParseResult::UnexpectedToken(unexpected_token) => {
                ParseResult::UnexpectedToken(unexpected_token)
            }
        }
    }
}

fn parse_else_clause<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    match token {
        Token::Else => parse_compound_list(lexer, token),
        Token::Elif => parse_compound_list(lexer, token),
        unexpected_token => ParseResult::UnexpectedToken(unexpected_token),
    }
}

fn parse_compound_list<R: Read>(lexer: &mut Lexer<R>, token: Token) -> ParseResult {
    match parse_and_or(lexer, token) {
        ParseResult::Success(ast, Token::Semicolon) => {
            let next_token = lexer.next();
            ParseResult::Success(ast, next_token)
        }
        e => e,
    }
}
