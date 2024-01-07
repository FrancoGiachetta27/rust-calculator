pub mod lexer;

use crate::lexer::*;
use std::fmt::Debug;
use Token::{TKId, TKNum, TKOprt, TKParenL, TKParenR, TKVar, EOF};

#[derive(Debug)]
pub enum GramItem {
    Id(String),
    Num(i32),
    Op(GramOp),
}

#[derive(Debug)]
pub enum GramOp {
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

    pub fn parse(input: &'a str) -> Result<i32, String> {
        let parser = Self::new(input);

        Ok(parser.parse_line()?)
    }
    // parse line
    fn parse_line(&self) -> Result<i32, String> {
        let token = self.scanner.next_token()?;

        match token {
            // TKVar => {
            //     self.scanner.match_token(token)?;
            //     self.scanner
            //         .match_token(TKId(self.scanner.get_lex().unwrap()))?;
            //     self.parse_var_def()?;
            // }
            TKId(_) | TKNum(_) | TKParenL => Ok(self.parse_exp()?),
            _ => return Err(format!("Syntax Error in parse_line -> {token:?}")),
        }
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

    fn parse_exp(&self) -> Result<i32, String> {
        let token = self.scanner.next_token()?;

        match token {
            TKId(_) => {
                self.scanner.match_token(token)?;

                match self.scanner.next_token().unwrap() {
                    // TKOprt(op) if op == "=" => {
                    //     self.scanner.match_token(TKOprt(op))?;
                    //     self.parse_exp()?;
                    // }
                    _ => Ok(self.parse_sum()?),
                }
            }
            TKNum(_) | TKParenL | EOF => Ok(self.parse_sum()?),
            _ => return Err(format!("Syntax Error in parse_exp -> {token:?}")),
        }
    }

    fn parse_sum(&self) -> Result<i32, String> {
        Ok(self.parse_term()? + self.parse_sum_()?)
    }

    fn parse_sum_(&self) -> Result<i32, String> {
        let token = self.scanner.next_token()?;

        match token {
            TKOprt(ref op) if op == "+" => {
                self.scanner.match_token(token)?;
                Ok(self.parse_exp()?)
            }
            TKOprt(ref op) if op == "-" => {
                self.scanner.match_token(token)?;
                let result = self.parse_exp()?;

                Ok(-result)
            }
            _ => Ok(0 as i32),
        }
    }

    fn parse_term(&self) -> Result<i32, String> {
        let lhs = self.parse_fact()?;
        self.parse_term_()?;

        Ok(lhs)
    }

    fn parse_term_(&self) -> Result<i32, String> {
        let token = self.scanner.next_token()?;

        match token {
            // TKOprt(ref op) if op == "*" => {
            //     self.scanner.match_token(token)?;
            //     self.parse_fact()?;
            //     self.parse_term_()?;
            // }
            // TKOprt(ref op) if op == "/" => {
            //     self.scanner.match_token(token)?;
            //     self.parse_fact()?;
            //     self.parse_term_()?;
            // }
            _ => Ok(0 as i32),
        }
    }

    fn parse_fact(&self) -> Result<i32, String> {
        let lhs = self.parse_prim()?;
        // self.parse_pow()?;

        Ok(lhs)
    }

    // fn parse_pow(&self) -> Result<(), String> {
    //     let token = self.scanner.next_token()?;

    //     match token {
    //         TKOprt(ref op) if op == "^" => {
    //             self.scanner.match_token(token)?;
    //             self.parse_fact()?;
    //         }
    //         _ => {}
    //     }

    //     Ok(())
    // }

    fn parse_prim(&self) -> Result<i32, String> {
        let token = self.scanner.next_token()?;

        match token {
            TKNum(num) => {
                self.scanner.match_token(token)?;
                println!("{num}");
                Ok(num)
            }
            // TKParenL => {
            //     self.scanner.match_token(token)?;
            //     self.parse_exp()?;
            //     self.scanner.match_token(TKParenR)?;
            // }
            _ => Ok(0 as i32),
        }
    }
}
