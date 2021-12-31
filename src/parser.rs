use crate::types::*;
use std::rc::Rc;

pub struct Parser {
    input: Vec<Token>,
    output: Vec<Definition>,
    current: Option<Token>,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        let t = tokens.pop();
        Parser {
            input: tokens,
            output: Vec::new(),
            current: t,
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.current = self.input.pop();
        self.current.clone()
    }

    pub fn run(mut self) -> Result<Vec<Definition>, String> {
        while self.current.is_some() {
            let def = self.parseDef()?;
            self.output.push(def);
        }
        Ok(self.output)
    }

    fn parseDef(&mut self) -> Result<Definition, String> {
        let tk = self.current.clone().ok_or("Unexpected EOF")?;
        match tk {
            Token::Number(n) => {
                self.next();
                Ok(Definition::Raw(Rc::new(Tree::Number(n))))
            }
            Token::Boolean(b) => {
                self.next();
                Ok(Definition::Raw(Rc::new(Tree::Boolean(b))))
            }
            Token::String(s) => {
                self.next();
                Ok(Definition::Raw(Rc::new(Tree::String(s))))
            }
            Token::Identifier(i) => {
                self.next();
                Ok(Definition::Raw(Rc::new(Tree::Identifier(i))))
            }
            Token::Delim('\'') => {
                let nx = self.next().ok_or("Unexpected EOF after '")?;
                self.next();
                match nx {
                    Token::Identifier(i) => Ok(Definition::Raw(Rc::new(Tree::Quote(i)))),
                    Token::Delim('(') => unimplemented!(), // parse list and pair
                    _ => Err(format!("Unexpected token {:?} after '", nx)),
                }
            }
            Token::Delim('(') => {
                let nx = self.next().ok_or("Unexpected EOF after (")?;
                match nx {
                    Token::Keyword(def) if def == "define" => unimplemented!(), // parse define body
                    _ => unimplemented!(), // parse function call
                }
            }
            _ => Err(format!("Unexpected start of {:?}", tk)),
        }
    }

    fn parse(&mut self) -> Result<AST, String> {
        unimplemented!()
    }
}
