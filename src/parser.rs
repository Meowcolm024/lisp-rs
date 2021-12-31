use crate::types::*;

pub struct Parser {
    input: Vec<Token>,
    output: Vec<Tree>,
    current: Option<Token>,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        let t = tokens.pop();
        Parser {
            input: tokens,
            output: Vec::new(),
            current: t,
        }
    }

    fn next(&mut self) -> Option<&Token> {
        self.current = self.input.pop();
        self.current.as_ref()
    }

    pub fn run(self) -> Vec<Tree> {
        self.output
    }
}
