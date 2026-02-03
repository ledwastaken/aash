mod ast;
mod lexer;
mod parser;

use ast::Ast;
use lexer::Lexer;
use parser::parse_input;

fn main() {
    // FIXME

    parse_execute_loop();
}

pub fn parse_execute_loop() {
    let mut lexer = Lexer::new();

    let ast = parse_input(lexer);

    loop {
        match ast {
            Ast::SimpleCommand(ref simple_command) => {
                // FIXME
            }
            Ast::Eof => break,
        }
    }

    println!("Hello, world!");
}
