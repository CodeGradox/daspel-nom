extern crate daspel_rs;
extern crate nom;

use daspel_rs::parser::run;
use nom::IResult;

#[test]
fn test_parens() {
    assert_eq!(run(b"(42)").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("(42)")));
    assert_eq!(run(b"(((4*10)+2))").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("(((4 * 10) + 2))")));
}

#[test]
fn test_unsiged_int() {
    assert_eq!(run(b"42").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("42")));
    assert_eq!(run(b"   42   ").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("42")));
}

#[test]
fn test_unsigned_real() {
    assert_eq!(run(b"4.14").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("4.14")));
    assert_eq!(run(b"3.14532").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("3.14532")));
}

#[test]
fn test_factor() {
    assert_eq!(run(b"-4.14").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("-4.14")));
    assert_eq!(run(b"  -  3.14532").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("-3.14532")));
    assert_eq!(run(b"  -  35").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("-35")));
}

#[test]
fn test_term() {
    assert_eq!(run(b"3*2").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("3 * 2")));
    assert_eq!(run(b"25   / 5").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("25 / 5")));
    assert_eq!(run(b"-5*5/-9").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("-5 * 5 / -9")));
}

#[test]
fn test_expr() {
    assert_eq!(run(b"3+2").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("3 + 2")));
    assert_eq!(run(b"3   -2").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("3 - 2")));
    assert_eq!(run(b"3+-2--2--2").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("3 + -2 - -2 - -2")));
    assert_eq!(run(b"(45--8)* 33 / (2+ 66)").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("(45 - -8) * 33 / (2 + 66)")));
}

#[test]
fn test_string() {
    assert_eq!(run(b"\"Hello, World!\"").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("\"Hello, World!\"")));
    assert_eq!(run(b"\"Hello\\\\/,\\n \tWorld!\"").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("\"Hello\\/,\n \tWorld!\"")));
}

#[test]
fn test_mix() {
    assert_eq!(run(b"33 + -(\"abc\" * 2)").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("33 + -(\"abc\" * 2)")));
}

#[test]
fn test_comments() {
    assert_eq!(run(b"\t33 #lettis\n   + # comment\n  # hmm \n 2").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("33 + 2")));
}

#[test]
fn test_comp_expr() {
    assert_eq!(run(b"true == (false != false)").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("true == (false != false)")));
}

#[test]
fn test_not_expr() {
    assert_eq!(run(b"!true != (!false)").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("!true != (!false)")));
}

#[test]
fn test_and_expr() {
    assert_eq!(run(b"(false&false)&true").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("(false & false) & true")));
}

#[test]
fn test_or_expr() {
    assert_eq!(run(b" true&true|(!false|true)").map(|x| format!("{}", x)),
               IResult::Done(&b""[..], String::from("true & true | (!false | true)")));
}

#[test]
fn test_identity() {
    assert_eq!(run(b"a + -b * 2 / -(-x*-y)").map(|x| format!("{}", x)),
        IResult::Done(&b""[..], String::from("a + -b * 2 / -(-x * -y)")));
}
