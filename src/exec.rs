use std::process::Command;

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
        Ast::None => 0,
    }
}

fn execute_simple_command(program: String, args: Vec<String>) -> i32 {
    let status = Command::new(program)
        .args(args)
        .status()
        .expect("failed to execute");

    match status.code() {
        Some(code) => code,
        None => 130,
    }
}
