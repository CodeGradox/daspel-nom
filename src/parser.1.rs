use nom::{IResult, digit};

use std::str;
use std::str::FromStr;

use ast;

// Parsens an expression wrapped by parenthesis: '(' expr ')'
named!(parens<ast::Expr>, ws!(
    delimited!(
        tag!("("), expr, tag!(")")
    )
));

// Takes a digit and parses it into an i32
named!(unsiged_int<ast::Expr>, alt!(
    map!(
        map_res!(
            map_res!(
                ws!(digit),
                str::from_utf8
            ),
            FromStr::from_str
        ),
        |n: i32| ast::Expr::Lit(ast::Lit(n))
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
        if sign.is_some() { ast::Expr::UnaryOp(ast::UnOp::Neg,value) } else { value }
    }
));

// Parse terms: factor (('+' | '-') factor)*
named!(term<ast::Expr>, do_parse!(
    val: factor >>
    res: fold_many0!(
        pair!(alt!(tag!("*") | tag!("/")), factor),
        val,
        |acc, (op, val): (&[u8], ast::Expr)| {
            if (op[0] as char) == '*' { acc * val } else { acc / val }
        }
    ) >>
    (res)
));

// Parse expression: term  (('+' | '-') term)*
named!(pub expr<i32>, do_parse!(
    val: term >>
    res: fold_many0!(
        pair!(alt!(tag!("+") | tag!("-")), term),
        val,
        |acc, (op, val): (&[u8], i32)| {
            if (op[0] as char) == '+' { acc + val } else { acc - val }
        }
    ) >>
    (res)
));

#[test]
fn test_parens() {
    assert_eq!(parens(b"(42)"), IResult::Done(&b""[..], 42));
    assert_eq!(parens(b"(((4*10)+2))"), IResult::Done(&b""[..], 42));
}

#[test]
fn test_unsiged_int() {
    assert_eq!(unsiged_int(b"42"), IResult::Done(&b""[..], 42));
    assert_eq!(unsiged_int(b"  42  "), IResult::Done(&b""[..], 42));
    assert!(unsiged_int(b"nan").is_err());
}

#[test]
fn test_sign() {
    assert_eq!(factor(b"-42"), IResult::Done(&b""[..], -42));
    assert_eq!(factor(b" -   42"), IResult::Done(&b""[..], -42));
}

#[test]
fn test_term() {
    assert_eq!(term(b"3*2"), IResult::Done(&b""[..], 6));
    assert_eq!(term(b"25 / 5"), IResult::Done(&b""[..], 5));
    assert_eq!(term(b"3/3*2*10"), IResult::Done(&b""[..], 20));
    assert_eq!(term(b"-5*-5/5"), IResult::Done(&b""[..], 5));
}

#[test]
fn test_expr() {
    assert_eq!(expr(b"3 + 2"), IResult::Done(&b""[..], 5));
    assert_eq!(expr(b"3 - 2"), IResult::Done(&b""[..], 1));
    assert_eq!(expr(b" 3 -   - 2"), IResult::Done(&b""[..], 5));
    assert_eq!(expr(b"-9+-2--3"), IResult::Done(&b""[..], -8));
    assert_eq!(expr(b"12+34-9-9+0"), IResult::Done(&b""[..], 28));
    assert_eq!(expr(b"3 + 2 * 4"), IResult::Done(&b""[..], 11));
}