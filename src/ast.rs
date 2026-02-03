pub mod simple_command;

pub use self::simple_command::SimpleCommand;

pub enum Ast {
    SimpleCommand(SimpleCommand),
    Eof,
}
