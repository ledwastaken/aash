#[derive(Debug)]
pub enum Ast {
    SimpleCommand {
        program: String,
        args: Vec<String>,
    },
    List(Vec<Ast>),
    IfCommand {
        condition: Box<Ast>,
        then_branch: Box<Ast>,
        else_branch: Option<Box<Ast>>,
    },
    None,
}
