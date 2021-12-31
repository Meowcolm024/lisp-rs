use std::rc::Rc;

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

pub type AST = Rc<Tree>;

#[derive(Debug, PartialEq, Eq)]
pub enum Tree {
    Identifier(String),     // x
    List(Vec<AST>),         // (list 1 2 3)
    Pair(AST, AST),         // (cons 1 2)
    If(AST, AST, AST),      // (if p t e)
    Cond(Vec<(AST, AST)>),  // (cond (p e) ...)
    Lambda(Vec<AST>, AST),  // (lambda (a b c) body)
    Call(AST, Vec<AST>),    // (f x y z)
}
