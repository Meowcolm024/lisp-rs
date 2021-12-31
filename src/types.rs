#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Number(u32),
    String(String),
    Boolean(bool),
    Quote(String),
    Keyword(String),
    Delim(char),
    Error(String),
    Space,
}
