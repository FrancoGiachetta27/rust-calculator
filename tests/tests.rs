use rust_calculator::lexer::{Scanner, Token};
use rust_calculator::Parser;

// #[test]
// fn check_lexer() {
//     let mut vec = vec![];
//     let scanner = Scanner::new("(1+1)*(2^2)");
//     let mut tkn = scanner.next_token().unwrap();

//     while tkn != Token::EOF {
//         vec.push(tkn);
//         tkn = scanner.next_token().unwrap();
//     }

//     let vec_result = vec![
//         Token::TKParenL,
//         Token::TKNum(1),
//         Token::TKOprt("+".to_string()),
//         Token::TKNum(1),
//         Token::TKParenR,
//         Token::TKOprt("*".to_string()),
//         Token::TKParenL,
//         Token::TKNum(2),
//         Token::TKOprt("^".to_string()),
//         Token::TKNum(2),
//         Token::TKParenR,
//     ];

//     assert_eq!(vec_result, vec);
// }

#[test]
fn one_number() {
    assert_eq!(2.0, Parser::parse("2").unwrap())
}

#[test]
fn sum_multiple_numbers() {
    let sum = Parser::parse("2+4+6+8+10").unwrap();

    assert_eq!(30.0, sum);
}

#[test]
fn sub_multiple_numbers() {
    let sub = Parser::parse("1-3-5").unwrap();

    assert_eq!(-7.0, sub);
}

#[test]
fn div_multiple_numbers() {
    let sub = Parser::parse("2/2/2/2").unwrap();

    assert_eq!(1.0 / 4.0, sub);
}

#[test]
fn mul_multiple_numbers() {
    let sub = Parser::parse("2*2*2*2").unwrap();

    assert_eq!(16.0, sub);
}

#[test]
fn pow_multiple_numbers() {
    let pow = Parser::parse("2^2^2^2").unwrap();

    assert_eq!(65536.0, pow);
}

#[test]
fn combined_op() {
    let sum = Parser::parse("2 + 6*3/2 + 5*4 + 1").unwrap();
    let sum2 = Parser::parse("1 - 2*3*5/2 - 4/2*2 + 5*3*2").unwrap();
    let sum3 = Parser::parse("1^2+7*3^5-120^2").unwrap();
    
    assert_eq!(32.0, sum);
    assert_eq!(12.0, sum2);
    assert_eq!(-12698.0, sum3);
}

#[test]
fn sum_error() {
    let sum = Parser::parse("1--1");
    let sum2 = Parser::parse("-2-2");

    assert!(sum.is_err());
    assert!(sum2.is_err());
}
