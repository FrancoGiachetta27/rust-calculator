use std::io;

use parser::Parser;

pub mod utils;
pub mod lexer;
pub mod parser;

fn main() {
    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        println!("{}", Parser::parse(&input).unwrap());
    }

}
