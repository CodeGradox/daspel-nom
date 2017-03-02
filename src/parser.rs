use nom::{IResult, ErrorKind, digit};
use std::str::FromStr;
use std::str;
use ast;

// Parser generator for skipping whitespaces and comments.
#[macro_export]
macro_rules! wsc {
    ($i:expr, $($args:tt)*) => {{
        sep!($i, skip_ws_comment, $($args)*)
    }}
}

// Entry point for the parser.
named!(pub run<ast::Expr>, call!(or_expr));

// Parse or (`|`) expression: and_expr (('|') and_expr)*
named!(or_expr<ast::Expr>, do_parse!(
    val: and_expr >>
    reminder: many0!(
        do_parse!(tag!("|") >> and: and_expr >> (ast::BinOp::Or, and))
    ) >>
    (fold_expr(val, reminder))
));

// Parse and (`&`) expression: not_expr (('&') not_expr)*
named!(and_expr<ast::Expr>, do_parse!(
    val: not_expr >>
    reminder: many0!(
        do_parse!(tag!("&") >> not: not_expr >> (ast::BinOp::And, not))
    ) >>
    (fold_expr(val, reminder))
));

// Parse logical not expression: `!`? comp_expr
//
// This one is a bit dumb because we can't have a comp_expr be
// followed by a `!`. The following is illegal: x == !x
// Instead we must write: x == (!x)
// Maybe we can change how the parsing happens at a later
// stage and just check the UnOp when we check the type.
named!(not_expr<ast::Expr>, map!(
    pair!(
        wsc!(opt!(tag!("!"))),
        comp_expr
    ),
    |(sign, value): (Option<&[u8]>, ast::Expr)| {
        if sign.is_some() {
            ast::Expr::UnaryOp(ast::UnOp::Not, Box::new(value))
        } else {
            value
        }
    }
));

// Parse comparison expression: not_expr (('&') not_expr)*
named!(comp_expr<ast::Expr>, do_parse!(
    val: expr >>
    reminder: many0!(
        alt!(
            do_parse!(tag!("==") >> eq: expr >> (ast::BinOp::Eq, eq)) |
            do_parse!(tag!("!=") >> ne: expr >> (ast::BinOp::Ne, ne)) |
            do_parse!(tag!(">")  >> gt: expr >> (ast::BinOp::Gt, gt)) |
            do_parse!(tag!(">=") >> ge: expr >> (ast::BinOp::Ge, ge)) |
            do_parse!(tag!("<")  >> lt: expr >> (ast::BinOp::Lt, lt)) |
            do_parse!(tag!("<=") >> le: expr >> (ast::BinOp::Le, le))
        )
    ) >>
    (fold_expr(val, reminder))
));

// Parse expression: term  (('+' | '-') term)*
named!(expr<ast::Expr>, do_parse!(
    val: term >>
    reminder: many0!(
        alt!(
            do_parse!(tag!("+") >> add: term >> (ast::BinOp::Add, add)) |
            do_parse!(tag!("-") >> sub: term >> (ast::BinOp::Sub, sub))
        )
    ) >>
    (fold_expr(val, reminder))
));

// Parse terms: factor (('*' | '/') factor)*
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

// Parses a factor: '-'? int_lit
named!(factor<ast::Expr>, map!(
    pair!(
        wsc!(opt!(tag!("-"))),
        alt!(
            unsigned_real |
            unsigned_int |
            parens |
            do_parse!(wsc!(tag!("true")) >> (ast::Expr::Lit(ast::Lit::Bool(true)))) |
            do_parse!(wsc!(tag!("false")) >> (ast::Expr::Lit(ast::Lit::Bool(false)))) |
            do_parse!(wsc!(tag!("nil")) >> (ast::Expr::Lit(ast::Lit::Nil))) |
            wsc!(string) |
            wsc!(ident)
        )
    ),
    |(sign, value): (Option<&[u8]>, ast::Expr)| {
        if sign.is_some() {
            ast::Expr::UnaryOp(ast::UnOp::Neg, Box::new(value))
        } else {
            value
        }
    }
));

// Takes a digit and parses it into an i32
//
// How it works:
// First, it tries to find a decimal number.
// complete! changes an incomplete find to an error.
// recognize! will return the full delimited match (as opposed to only return the ".").
// wsc! must be outside of recognize! or else it would try to parse a string
// with spaces, which would return an error.
// Then it maps the &[u8] to a str, then a f32, then finally an Expr.
// If it fails to find or parse a real, it will call parsens.
named!(unsigned_real<ast::Expr>, map!(
    map_res!(
        map_res!(
            wsc!(
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
));

// Takes a digit and parses it into an i32
//
// How it works:
// It finds a digit, then turns the &[u8] into a str.
// Then the str is parsed into an int and is finally returned as an Expr::Lit.
// If it fails to find or parse a real, it will call parsens.
named!(unsigned_int<ast::Expr>, map!(
    map_res!(
        map_res!(
            wsc!(digit),
            str::from_utf8
        ),
        FromStr::from_str
    ),
    |n: i32| ast::Expr::Lit(ast::Lit::Int(n))
));

// Parsens an expression wrapped by parenthesis: '(' expr ')'
named!(parens<ast::Expr>, wsc!(
    delimited!(
        tag!("("),
        map!(map!(or_expr, Box::new), ast::Expr::Paren),
        tag!(")")
    )
));

// This parser does NOT allow newlines in strings.
named!(string<ast::Expr>, delimited!(
    tag!("\""),
    parse_string,
    tag!("\"")
));

// Parser for identities (names).
named!(ident<ast::Expr>, map!(
    verify!(
        map_res!(
            take_till!(invalid_indent_char),
            str::from_utf8
        ), |s: &str| !s.is_empty()
    ),
    |s: &str| ast::Expr::Ident(String::from(s))
));

// Checks is input is a non character or number.
fn invalid_indent_char(input: u8) -> bool {
    " \t\r\n\"!?.,;:&=<>+-*/@(){}[]^~\\%$".find(input as char).is_some()
}

// Folds a series of operators and expressions to a AST.
fn fold_expr(initial: ast::Expr, remainder: Vec<(ast::BinOp, ast::Expr)>) -> ast::Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        ast::Expr::BinaryOp(oper, Box::new(acc), Box::new(expr))
    })
}

/// Skips whitespaces and comments.
///
/// Comments start with a `#` and end with a newline.
/// Anything inside a comment is ignored.
fn skip_ws_comment(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let mut idx = 0;
    let limit = input.len();
    while idx < limit {
        match input[idx] {
            b' ' | b'\t' | b'\r' | b'\n' => idx += 1,
            b'#' => {
                while idx < limit && input[idx] != b'\n' {
                    idx += 1;
                }
            }
            _ => break,
        }
    }
    IResult::Done(&input[idx..], &input[0..0])
}

// Helper function for parsing strings.
// Strings must start and end on the same line.
fn parse_string(input: &[u8]) -> IResult<&[u8], ast::Expr> {
    let mut s = String::new();
    let chars = match str::from_utf8(input) {
        Ok(ok) => ok,
        Err(_) => return IResult::Error(ErrorKind::IsNotStr),
    };
    let mut chars = chars.char_indices();
    while let Some((byte_offset, ch)) = chars.next() {
        match ch {
            '"' => {
                let expr = ast::Expr::Lit(ast::Lit::Str(s));
                return IResult::Done(&input[byte_offset..], expr);
            }
            '\\' => {
                match chars.next() {
                    Some((_, 'n')) => s.push('\n'),
                    Some((_, 'r')) => s.push('\r'),
                    Some((_, 't')) => s.push('\t'),
                    Some((_, '"')) => s.push('"'),
                    Some((_, '\'')) => s.push('\''),
                    Some((_, '\\')) => s.push('\\'),
                    Some((_, '\n')) | Some((_, '\r')) => return IResult::Error(ErrorKind::IsNotStr),
                    _ => break,
                }
            }
            '\r' | '\n' => return IResult::Error(ErrorKind::IsNotStr),
            ch => {
                s.push(ch);
            }
        }
    }
    IResult::Error(ErrorKind::IsNotStr)
}