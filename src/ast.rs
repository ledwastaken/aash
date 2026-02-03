pub mod simple_command;

pub use simple_command::SimpleCommand;

pub enum Ast {
    SimpleCommand(SimpleCommand),
}
