use crate::types::*;

use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a mut Chars<'a>,
    output: Vec<Token>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a mut Chars<'a>) -> Self {
        let c = s.next();
        Lexer {
            input: s,
            output: Vec::new(),
            current: c,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.current = self.input.next();
        self.current
    }

    pub fn run(mut self) -> Result<Vec<Token>, String> {
        while let Some(tk) = self.current {
            let token = match tk {
                '"' => self.string(),
                k if isDelim(k) => self.delim(k),
                ';' => self.comment(),
                x if x.is_ascii_digit() => self.number(x),
                x if x.is_whitespace() => {
                    self.next();
                    Token::Space
                }
                '#' => self.boolean(),
                w if w.is_ascii_alphanumeric() => self.general(w),
                _ => Token::Error(tk.to_string()),
            };
            if let Token::Error(msg) = &token {
                return Err(msg.to_string());
            } else {
                self.output.push(token)
            }
        }
        Ok(self
            .output
            .into_iter()
            .filter(|x| *x != Token::Space)
            .collect())
    }

    fn string(&mut self) -> Token {
        let mut acc: Vec<char> = Vec::new();
        while let Some(i) = self.next() {
            if i == '"' {
                return Token::String(acc.into_iter().collect());
            } else {
                acc.push(i);
            }
        }
        let t: String = acc.into_iter().collect();
        Token::Error("\"".to_string() + &t)
    }

    fn number(&mut self, it: char) -> Token {
        let mut acc: Vec<char> = vec![it];
        while let Some(i) = self.next() {
            if i.is_ascii_digit() {
                acc.push(i);
            } else {
                break;
            }
        }
        let s: String = acc.into_iter().collect();
        Token::Number(s.parse::<u32>().unwrap())
    }

    fn general(&mut self, it: char) -> Token {
        let mut acc: Vec<char> = vec![it];
        while let Some(i) = self.next() {
            if i.is_alphanumeric() || "-?!".contains(i) {
                acc.push(i)
            } else {
                break;
            }
        }
        let s: String = acc.into_iter().collect();
        if isKeyword(&s) {
            Token::Keyword(s)
        } else {
            Token::Identifier(s)
        }
    }

    fn comment(&mut self) -> Token {
        while let Some(i) = self.next() {
            if i == '\n' {
                break;
            }
        }
        Token::Space
    }

    fn boolean(&mut self) -> Token {
        let b = self.next();
        self.next();
        match b {
            Some('t') => Token::Boolean(true),
            Some('f') => Token::Boolean(false),
            k => Token::Error(format!("{:?}", k)),
        }
    }

    fn delim(&mut self, it: char) -> Token {
        self.next();
        Token::Delim(it)
    }
}

fn isKeyword(word: &String) -> bool {
    let keywords = ["if", "cond", "define", "lambda"];
    keywords.contains(&word.as_str())
}

fn isDelim(d: char) -> bool {
    let delims = "()+-*/=<>'.";
    delims.contains(d)
}
