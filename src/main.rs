mod ast;
mod exec;
mod lexer;
mod parser;

use std::fs::File;
use std::io::{Cursor, Read};

use ast::Ast;
use exec::execute_simple_command;
use lexer::Lexer;
use parser::parse_input;

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap();

    match args.next() {
        Some(arg) => match arg.as_str() {
            "-c" => match args.next() {
                Some(src) => parse_execute_loop(&mut Cursor::new(src)),
                None => {
                    eprintln!("{}: -c: option requires an argument", program_name);
                    std::process::exit(2);
                }
            },
            _ => parse_execute_loop(&mut File::open(arg).unwrap()),
        },
        None => parse_execute_loop(&mut std::io::stdin()),
    }
}

pub fn parse_execute_loop<R: Read>(src: &mut R) {
    let mut lexer = Lexer::new();

    let ast = parse_input(lexer);

    loop {
        match ast {
            Ast::SimpleCommand(ref simple_command) => {
                execute_simple_command(simple_command);
            }
            Ast::Eof => break,
        }
    }

    println!("Hello, world!");
}
