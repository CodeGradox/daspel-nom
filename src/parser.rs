use nom::{IResult, digit};

use std::str;
use std::str::FromStr;

named!(parens<i32>, ws!( delimited!( tag!("("), expr, tag!(")") ) ) );

// Takes a digit and parses it into an i32
named!(factor<i32>,
    alt!(
        map_res!(
            map_res!(
                ws!(digit),
                str::from_utf8
            ),
            FromStr::from_str
        )
        | parens
    )
);

named!(sign<i32>,
    map!(
        pair!(
            ws!(opt!(tag!("-"))),
            factor
        ),
        |(sign, value): (Option<&[u8]>, i32)| {
            if sign.is_some() { -value } else { value }
        }
    )
);

// Parse terms (* and /)
named!(pub term<i32>,
    do_parse!(
        val: sign >>
        res: fold_many0!(
            pair!(alt!(tag!("*") | tag!("/")), sign),
            val,
            |acc, (op, val): (&[u8], i32)| {
                if (op[0] as char) == '*' { acc * val } else { acc / val }
            }
        ) >>
        (res)
    )
);

// Parse expression (+ and -)
named!(pub expr<i32>,
    do_parse!(
        val: term >>
        res: fold_many0!(
            pair!(alt!(tag!("+") | tag!("-")), term),
            val,
            |acc, (op, val): (&[u8], i32)| {
                if (op[0] as char) == '+' { acc + val } else { acc - val }
            }
        ) >>
        (res)
    )
);

#[test]
fn test_parens() {
    assert_eq!(factor(b"(42)"), IResult::Done(&b""[..], 42));
    assert_eq!(factor(b"(((4*10)+2))"), IResult::Done(&b""[..], 42));
}

#[test]
fn test_factor() {
    assert_eq!(factor(b"42"), IResult::Done(&b""[..], 42));
    assert_eq!(factor(b"  42  "), IResult::Done(&b""[..], 42));
    assert!(factor(b"nan").is_err());
}

#[test]
fn test_sign() {
    assert_eq!(sign(b"-42"), IResult::Done(&b""[..], -42));
    assert_eq!(sign(b" -   42"), IResult::Done(&b""[..], -42));
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