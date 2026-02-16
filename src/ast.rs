#[derive(Debug)]
pub enum Ast {
    SimpleCommand { program: String, args: Vec<String> },
    List(Vec<Ast>),
    None,
}
