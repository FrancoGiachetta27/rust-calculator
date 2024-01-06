use crate::lexer::*;
use std::fmt::Debug;
use Token::{TKId, TKNum, TKOprt, TKParenL, TKParenR, TKVar};

#[derive(Debug)]
pub enum GramItem {
    Id(String),
    Num(i32),
}

pub enum GramOp{
    Eql,
    Sum,
    Sub,
    Mul,
    Div,
    Pow,
} 

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
        }
    }

    pub fn parse(input: &'a str) -> Result<String, String> {
        let parser = Self::new(input);

        parser.parse_line()?;

        Ok("Successfully parsed".to_string())
    }
    // parse line
    fn parse_line(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKVar => {
                self.scanner.match_token(token)?;
                self.scanner
                    .match_token(TKId(self.scanner.get_lex().unwrap()))?;
                self.parse_var_def()?;
            }
            TKId(_) | TKNum(_) | TKParenL => self.parse_exp()?,
            _ => return Err(format!("Syntax Error in parse_line -> {token:?}")),
        }

        Ok(())
    }

    fn parse_var_def(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKOprt(ref op) if op == "=" => {
                self.scanner.match_token(token)?;
                self.parse_exp()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_exp(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKId(_) => {
                self.scanner.match_token(token)?;

                match self.scanner.next_token().unwrap() {
                    TKOprt(op) if op == "=" => {
                        self.scanner.match_token(TKOprt(op))?;
                        self.parse_exp()?;
                    }
                    _ => {
                        self.parse_sum()?;
                    }
                }
            }
            TKNum(_) | TKParenL => self.parse_sum()?,
            _ => return Err(format!("Syntax Error in parse_exp -> {token:?}")),
        }

        Ok(())
    }

    fn parse_sum(&self) -> Result<(), String> {
        self.parse_term()?;
        self.parse_sum_()?;

        Ok(())
    }

    fn parse_sum_(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKOprt(ref op) if op == "+" => {
                self.scanner.match_token(token)?;
                self.parse_exp()?;
            }
            TKOprt(ref op) if op == "-" => {
                self.scanner.match_token(token)?;
                self.parse_exp()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_term(&self) -> Result<(), String> {
        self.parse_fact()?;
        self.parse_term_()?;

        Ok(())
    }

    fn parse_term_(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKOprt(ref op) if op == "*" => {
                self.scanner.match_token(token)?;
                self.parse_fact()?;
                self.parse_term_()?;
            }
            TKOprt(ref op) if op == "/" => {
                self.scanner.match_token(token)?;
                self.parse_fact()?;
                self.parse_term_()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_fact(&self) -> Result<(), String> {
        self.parse_prim()?;
        self.parse_pow()?;

        Ok(())
    }

    fn parse_pow(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKOprt(ref op) if op == "^" => {
                self.scanner.match_token(token)?;
                self.parse_fact()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_prim(&self) -> Result<(), String> {
        let token = self.scanner.next_token()?;

        match token {
            TKNum(_) => {
                self.scanner.match_token(token)?;
            }
            TKParenL => {
                self.scanner.match_token(token)?;
                self.parse_exp()?;
                self.scanner.match_token(TKParenR)?;
            }
            _ => {}
        }

        Ok(())
    }
}
