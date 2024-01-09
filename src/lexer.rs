use std::{
    cell::{Cell, RefCell},
    iter::Peekable,
    str::Chars,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    TKNum(f32),
    TKParenL,
    TKParenR,
    TKId(String),
    TKOprt(String),
    TKVar,
    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Init,
    Ident,
    IdentRec,
    Num,
    NumRec,
    ParenL,
    ParenLRec,
    ParenRRec,
    ParenR,
    Blank,
    Op,
    OpRec,
    Oth,
    End,
    Error(LexErr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexErr {
    IdErr,
    NumErr,
    UexpErr,
}

pub struct Scanner<'a> {
    it: RefCell<Peekable<Chars<'a>>>,
    lex_item: RefCell<String>,
    state: Cell<State>,
    matched: Cell<bool>,
    next_token: RefCell<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let it = input.chars().peekable();

        Self {
            it: RefCell::new(it),
            lex_item: RefCell::new(String::new()),
            state: Cell::new(State::Init),
            matched: Cell::new(true),
            next_token: RefCell::new(Token::EOF),
        }
    }

    pub fn get_lex(&self) -> Option<String> {
        let lex_item = self.lex_item.borrow();

        if lex_item.is_empty() {
            None
        } else {
            Some(lex_item.to_string())
        }
    }

    pub fn next_token(&self) -> Result<Token, String> {
        let result = self.calculate_next_token();

        if let Ok(ref tkn) = result {
            *self.next_token.borrow_mut() = tkn.to_owned();
        }

        result
    }

    // get the result token or lexical error
    pub fn match_token(&self, token: Token) -> Result<(), String> {
        let next_token = self.next_token()?;

        if token == next_token {
            self.matched.set(true);
            Ok(())
        } else {
            Err(format!(
                "Syntax Error: expected token: {token:?} but found token: {next_token:?}"
            ))
        }
    }

    // get the state based on the character
    fn lex(c: &char) -> State {
        match *c {
            '0'..='9' => State::Num,
            'a'..='z' | 'A'..='Z' => State::Ident,
            '(' => State::ParenL,
            ')' => State::ParenR,
            '+' | '-' | '*' | '/' | '^' | '=' => State::Op,
            ' ' | '\n' => State::Blank,
            _ => State::Oth,
        }
    }

    // returns the next token
    pub fn calculate_next_token(&self) -> Result<Token, String> {
        if self.matched.get() {
            *self.lex_item.borrow_mut() = String::new();
            self.matched.set(false);

            if let Some(state) = self.scan() {
                return match state {
                    State::Init => Ok(Token::EOF),
                    State::Ident => Ok(Token::TKId(self.lex_item.borrow().to_string())),
                    State::Num => Ok(Token::TKNum(self.lex_item.borrow().parse::<f32>().unwrap())),
                    State::ParenL => Ok(Token::TKParenL),
                    State::ParenR => Ok(Token::TKParenR),
                    State::Op => Ok(Token::TKOprt(self.lex_item.borrow().to_string())),
                    State::Error(LexErr::IdErr) => Err(format!(
                        "Lexical error: invalid identifier -> {}",
                        self.lex_item.borrow()
                    )),
                    State::Error(LexErr::NumErr) => Err(format!(
                        "Lexical error: invalid number -> {}",
                        self.lex_item.borrow()
                    )),
                    _ => Err(format!(
                        "Lexical error: unexpected string -> {}",
                        self.lex_item.borrow()
                    )),
                };
            } else {
                return Ok(Token::EOF);
            }
        }

        Ok(self.next_token.borrow().to_owned())
    }

    fn next_state(&self, state: State) -> State {
        let curr_state = self.state.get();

        match (curr_state, state) {
            // Errors
            (State::Init, State::Oth) => State::Error(LexErr::UexpErr),
            (State::Num, State::Ident) => State::Error(LexErr::NumErr),
            // Init
            (State::Init, State::Blank) => State::Init,
            // Ident
            (State::Ident, State::Ident | State::Num) => curr_state,
            (State::Ident, _) => State::IdentRec,
            // Num
            (State::Num, State::Num) => State::Num,
            (State::Num, _) => State::NumRec,
            // Parens
            (State::ParenL, _) => State::ParenLRec,
            (State::ParenR, _) => State::ParenRRec,
            // Ops
            (State::Op, _) => State::OpRec,
            _ => state,
        }
    }

    fn check_errors(&self, state_read: State) -> State {
        let curr_state = self.state.get();

        match (curr_state, state_read) {
            (State::Num, State::Ident | State::Oth) => State::Error(LexErr::NumErr),

            _ => curr_state,
        }
    }

    fn accept_token(state: State) -> bool {
        match state {
            State::IdentRec => true,
            State::NumRec => true,
            State::ParenLRec => true,
            State::ParenRRec => true,
            State::OpRec => true,
            State::Blank => true,
            _ => false,
        }
    }

    fn scan(&self) -> Option<State> {
        let mut next_state: State;
        let mut iter = self.it.borrow_mut();
        let mut state_read: State;

        loop {
            match iter.peek() {
                Some(c) => {
                    state_read = Self::lex(&c);
                    next_state = self.next_state(state_read);
                    if Self::accept_token(next_state) {
                        let last_state = if let State::IdentRec | State::NumRec = next_state {
                            self.check_errors(state_read)
                        } else {
                            self.state.get()
                        };
                        self.state.set(State::Init);

                        return Some(last_state);
                    } else {
                        self.state.set(next_state);
                        if !c.is_ascii_whitespace() {
                            self.lex_item.borrow_mut().push(*c)
                        };
                    }
                    iter.next();
                }
                None => {
                    let last_state = self.check_errors(State::End);
                    self.state.set(State::Init);

                    return Some(last_state);
                }
            }
        }
    }
}
