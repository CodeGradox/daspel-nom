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
//
// How it works:
// First, it tries to find a decimal number.
// complete! changes an incomplete find to an error.
// recognize will return the full delimited (as it would only return the ".").
// ws! must be outside of recognize, or else it would try to parse a string
// with spaces, which would return an error.
// Then it maps the &[u8] to a str, then a f32, then finally an Expr.
named!(pub unsigned_real<ast::Expr>, alt_complete!(
    map!(
        map_res!(
            map_res!(
                ws!(
                    recognize!(
                        complete!(
                            delimited!(digit, tag!("."), digit)
                        )
                    )
                ),
                str::from_utf8
            ),
            FromStr::from_str
        ),
        |r: f32| ast::Expr::Lit(ast::Lit::Real(r))
    )
    | parens
));

// Takes a digit and parses it into an i32
named!(unsigned_int<ast::Expr>, alt_complete!(
    map!(
        map_res!(
            map_res!(
                ws!(digit),
                str::from_utf8
            ),
            FromStr::from_str
        ),
        |n: i32| ast::Expr::Lit(ast::Lit::Int(n))
    )
    | parens
));

// Parses a factor: '-'? int_lit
named!(factor<ast::Expr>, map!(
    pair!(
        ws!(opt!(tag!("-"))),
        alt!(unsigned_real | unsigned_int)
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

#[test]
fn test_parens() {
    assert_eq!(parens(b"(42)").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("(42)")));
    assert_eq!(parens(b"(((4*10)+2))").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("(((4 * 10) + 2))")));
}

#[test]
fn test_unsiged_int() {
    assert_eq!(unsigned_int(b"42").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("42")));
    assert_eq!(unsigned_int(b"   42   ").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("42")));
    assert!(unsigned_int(b"nan").is_err());
}

#[test]
fn test_unsigned_real() {
    assert_eq!(unsigned_real(b"4.14").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("4.14")));
    assert_eq!(unsigned_real(b"3.14532").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("3.14532")));
}

#[test]
fn test_factor() {
    assert_eq!(factor(b"-4.14").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("-4.14")));
    assert_eq!(factor(b"  -  3.14532").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("-3.14532")));
    assert_eq!(factor(b"  -  35").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("-35")));
}

#[test]
fn test_term() {
    assert_eq!(term(b"3*2").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("3 * 2")));
    assert_eq!(term(b"25   / 5").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("25 / 5")));
    assert_eq!(term(b"-5*5/-9").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("-5 * 5 / -9")));
}

#[test]
fn test_expr() {
    assert_eq!(expr(b"3+2").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("3 + 2")));
    assert_eq!(expr(b"3   -2").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("3 - 2")));
    assert_eq!(expr(b"3+-2--2--2").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("3 + -2 - -2 - -2")));
    assert_eq!(expr(b"(45--8)* 33 / (2+ 66)").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("(45 - -8) * 33 / (2 + 66)")));
}