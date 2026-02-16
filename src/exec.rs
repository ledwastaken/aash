use std::process::Command;

use crate::ast::Ast;

pub fn execute_ast(ast: Ast) {
    match ast {
        Ast::SimpleCommand { program, args } => execute_simple_command(program, args),
        Ast::List(commands) => {
            for cmd in commands {
                execute_ast(cmd);
            }
        }
        Ast::None => (),
    }
}

fn execute_simple_command(program: String, args: Vec<String>) {
    let _ = Command::new(program).args(args).spawn();
}
