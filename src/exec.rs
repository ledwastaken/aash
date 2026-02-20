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
    let mut start = 0;

    if args.first().map(|s| s.as_str()) == Some("-n") {
        newline = false;
        start = 1;
    }

    let output = args[start..].join(" ");

    if newline {
        println!("{}", output);
    } else {
        print!("{}", output);
        std::io::stdout().flush().unwrap();
    }

    0
}
