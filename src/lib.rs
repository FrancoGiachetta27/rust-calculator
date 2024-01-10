pub mod lexer;

use crate::lexer::*;
use Token::{TKId, TKNum, TKOprt, TKParenL, TKParenR, EOF};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
        }
    }

    pub fn parse(input: &'a str) -> Result<f32, String> {
        let parser = Self::new(input);

        Ok(parser.parse_line()?)
    }

    // parse line
    fn parse_line(&self) -> Result<f32, String> {
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

    fn parse_exp(&self) -> Result<f32, String> {
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

    fn parse_sum(&self) -> Result<f32, String> {
        Ok(self.parse_term()? + self.parse_sum_()?)
    }

    fn parse_sum_(&self) -> Result<f32, String> {
        let token = self.scanner.next_token()?;
        let mut result = 0.0;

        match token {
            TKOprt(ref op) if op == "+" => {
                self.scanner.match_token(token)?;

                result = result + self.parse_term()? + self.parse_sum_()?;
            }
            TKOprt(ref op) if op == "-" => {
                self.scanner.match_token(token)?;
                result = result - self.parse_term()? + self.parse_sum_()?;
            }
            _ => {}
        }

        Ok(result)
    }

    fn parse_term(&self) -> Result<f32, String> {
        Ok(self.parse_fact()? * self.parse_term_()?)
    }

    fn parse_term_(&self) -> Result<f32, String> {
        let token = self.scanner.next_token()?;
        let mut result = 1.0;

        match token {
            TKOprt(ref op) if op == "*" => {
                self.scanner.match_token(token)?;

                result = result * self.parse_fact()? * self.parse_term_()?;
            }
            TKOprt(ref op) if op == "/" => {
                self.scanner.match_token(token)?;

                result = result * (1.0 / self.parse_fact()?) * self.parse_term_()?;
            }
            _ => {}
        }

        Ok(result)
    }

    fn parse_fact(&self) -> Result<f32, String> {
        Ok(self.parse_prim()?.powf(self.parse_pow()?))
    }

    fn parse_pow(&self) -> Result<f32, String> {
        let token = self.scanner.next_token()?;

        match token {
            TKOprt(ref op) if op == "^" => {
                self.scanner.match_token(token)?;

                Ok(self.parse_fact()?)
            }
            _ => Ok(1 as f32),
        }
    }

    fn parse_prim(&self) -> Result<f32, String> {
        let token = self.scanner.next_token()?;

        match token {
            TKNum(num) => {
                self.scanner.match_token(token)?;

                Ok(num)
            }
            TKParenL => {
                self.scanner.match_token(token)?;
                let result = self.parse_exp()?;
                self.scanner.match_token(TKParenR)?;

                Ok(result)
            }
            _ => return Err(format!("Syntax Error in parse_prim -> {token:?}")),
        }
    }
}
