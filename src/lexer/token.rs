#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Keywords
    If,
    Then,
    Elif,
    Else,
    Fi,

    // Symbols
    Semicolon, // ;
    Newline,   // \n
    // Quote,     // '

    // Identifiers / literals
    Word(String),

    // End of input (very useful for parsers)
    Eof,
}
