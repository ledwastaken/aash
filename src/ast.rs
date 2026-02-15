#[derive(Debug)]
pub enum Ast {
    SimpleCommand { words: Vec<String> },
    None,
}
