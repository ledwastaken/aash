use std::{io::Write, process::Command};

use crate::ast::Ast;

pub fn execute_ast(ast: Ast) -> i32 {
    match ast {
        Ast::SimpleCommand { program, args } => execute_simple_command(program, args),
        Ast::List(commands) => {
            let mut code = 0;
            for cmd in commands {
                code = execute_ast(cmd);
            }
            code
        }
        Ast::IfCommand {
            condition,
            then_branch,
            else_branch,
        } => {
            if execute_ast(*condition) == 0 {
                execute_ast(*then_branch)
            } else if let Some(ast) = else_branch {
                execute_ast(*ast)
            } else {
                1
            }
        }
        Ast::None => 0,
    }
}

fn execute_simple_command(program: String, args: Vec<String>) -> i32 {
    match program.as_str() {
        "true" => 0,
        "false" => 1,
        "echo" => echo(args),
        _ => {
            let status = Command::new(program)
                .args(args)
                .status()
                .expect("failed to execute");

            match status.code() {
                Some(code) => code,
                None => 130,
            }
        }
    }
}

fn echo(args: Vec<String>) -> i32 {
    let mut newline = true;
    let mut interpret_escapes = false;
    let mut start = 0;

    for arg in &args {
        match arg.as_str() {
            "-n" => {
                newline = false;
                start += 1;
            }
            "-e" => {
                interpret_escapes = true;
                start += 1;
            }
            "-ne" | "-en" => {
                newline = false;
                interpret_escapes = true;
                start += 1;
            }
            _ => break,
        }
    }

    let output = args[start..].join(" ");

    let output = if interpret_escapes {
        interpret_escape_sequences(&output)
    } else {
        output
    };

    if newline {
        println!("{}", output);
    } else {
        print!("{}", output);
        std::io::stdout().flush().unwrap();
    }

    0
}

fn interpret_escape_sequences(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('0') => result.push('\0'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    result
}
