use std::env::Args;
use std::fs::File;
use std::io::{self, Cursor, Read};

use crate::exec::execute_ast;
use crate::lexer::Lexer;
use crate::parser::parse_input;

mod ast;
mod exec;
mod lexer;
mod parser;

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap_or("aash".to_string());

    let result = match args.next() {
        Some(arg) if arg == "-c" => handle_command_flag(args, &program_name),
        Some(filename) => handle_file_input(&filename, &program_name),
        None => parse_execute_loop(io::stdin()),
    };

    if let Err(err) = result {
        eprintln!("{program_name}: error: {err}");
        std::process::exit(1);
    }
}

fn handle_command_flag(mut args: Args, program_name: &str) -> io::Result<()> {
    match args.next() {
        Some(src) => parse_execute_loop(Cursor::new(src)),
        None => {
            eprintln!("{program_name}: -c: option requires an argument");
            std::process::exit(2);
        }
    }
}

fn handle_file_input(filename: &str, program_name: &str) -> io::Result<()> {
    match File::open(filename) {
        Ok(file) => parse_execute_loop(file),
        Err(_) => {
            eprintln!("{program_name}: {filename}: No such file or directory");
            std::process::exit(127);
        }
    }
}

fn parse_execute_loop<R: Read>(reader: R) -> io::Result<()> {
    let mut lexer = Lexer::new(reader);

    while let Some(ast) = parse_input(&mut lexer) {
        let _ = execute_ast(ast);
    }

    Ok(())
}
