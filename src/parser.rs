use nom::{IResult, digit};

use std::str;
use std::str::FromStr;

use ast;

// Parsens an expression wrapped by parenthesis: '(' expr ')'
named!(parens<ast::Expr>, ws!(
    delimited!(
        tag!("("),
        map!(map!(expr, Box::new), ast::Expr::Paren),
        tag!(")")
    )
));

// Takes a digit and parses it into an i32
named!(unsiged_int<ast::Expr>, alt_complete!(
    map!(
        map!(
            map_res!(
                map_res!(
                    ws!(digit),
                    str::from_utf8
                ),
                FromStr::from_str
            ),
            ast::Lit::Int
        ),
        ast::Expr::Lit
    )
    | parens
));

// Parses a factor: '-'? int_lit
named!(factor<ast::Expr>, map!(
    pair!(
        ws!(opt!(tag!("-"))),
        unsiged_int
    ),
    |(sign, value): (Option<&[u8]>, ast::Expr)| {
        if sign.is_some() {
            ast::Expr::UnaryOp(ast::UnOp::Neg, Box::new(value))
        } else {
            value
        }
    }
));

fn fold_expr(initial: ast::Expr, remainder: Vec<(ast::BinOp, ast::Expr)>) -> ast::Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        ast::Expr::BinaryOp(oper, Box::new(acc), Box::new(expr))
    })
}

// Parse terms: factor (('+' | '-') factor)*
named!(term<ast::Expr>, do_parse!(
    val: factor >>
    reminder: many0!(
        alt!(
            do_parse!(tag!("*") >> mul: factor >> (ast::BinOp::Mul, mul)) |
            do_parse!(tag!("/") >> div: factor >> (ast::BinOp::Div, div))
        )
    ) >>
    (fold_expr(val, reminder))
));

// Parse expression: term  (('+' | '-') term)*
named!(pub expr<ast::Expr>, do_parse!(
    val: term >>
    reminder: many0!(
        alt!(
            do_parse!(tag!("+") >> add: term >> (ast::BinOp::Add, add)) |
            do_parse!(tag!("-") >> sub: term >> (ast::BinOp::Sub, sub))
        )
    ) >>
    (fold_expr(val, reminder))
));

// #[test]
// fn test_parens() {
//     assert_eq!(parens(b"(42)"), IResult::Done(&b""[..], 42));
//     assert_eq!(parens(b"(((4*10)+2))"), IResult::Done(&b""[..], 42));
// }

// #[test]
// fn test_unsiged_int() {
//     assert_eq!(unsiged_int(b"42"), IResult::Done(&b""[..], 42));
//     assert_eq!(unsiged_int(b"  42  "), IResult::Done(&b""[..], 42));
//     assert!(unsiged_int(b"nan").is_err());
// }

// #[test]
// fn test_sign() {
//     assert_eq!(factor(b"-42"), IResult::Done(&b""[..], -42));
//     assert_eq!(factor(b" -   42"), IResult::Done(&b""[..], -42));
// }

// #[test]
// fn test_term() {
//     assert_eq!(term(b"3*2"), IResult::Done(&b""[..], 6));
//     assert_eq!(term(b"25 / 5"), IResult::Done(&b""[..], 5));
//     assert_eq!(term(b"3/3*2*10"), IResult::Done(&b""[..], 20));
//     assert_eq!(term(b"-5*-5/5"), IResult::Done(&b""[..], 5));
// }

// #[test]
// fn test_expr() {
//     assert_eq!(expr(b"3 + 2"), IResult::Done(&b""[..], 5));
//     assert_eq!(expr(b"3 - 2"), IResult::Done(&b""[..], 1));
//     assert_eq!(expr(b" 3 -   - 2"), IResult::Done(&b""[..], 5));
//     assert_eq!(expr(b"-9+-2--3"), IResult::Done(&b""[..], -8));
//     assert_eq!(expr(b"12+34-9-9+0"), IResult::Done(&b""[..], 28));
//     assert_eq!(expr(b"3 + 2 * 4"), IResult::Done(&b""[..], 11));
// }