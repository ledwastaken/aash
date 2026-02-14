pub mod simple_command;

pub use self::simple_command::SimpleCommand;

#[derive(Debug)]
pub enum Ast {
    SimpleCommand(SimpleCommand),
    None,
}
