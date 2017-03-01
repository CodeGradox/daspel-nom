use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Lit(Lit),
    BinaryOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>),
    Paren(Box<Expr>),
    List(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Ident(ref i) => fmt::Display::fmt(i, f),
            Lit(ref l) => fmt::Display::fmt(l, f),
            BinaryOp(ref op, ref e1, ref e2) => write!(f, "{} {} {}", e1, op.to_string(), e2),
            UnaryOp(ref op, ref e) => write!(f, "{}{}", op.to_string(), e),
            Paren(ref e) => write!(f, "({})", e),
            List(ref e) => write!(f, "[{:?}]", e),
        }
    }
}

#[derive(Debug)]
pub enum Lit {
    Int(i32),
    Real(f32),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Lit::*;
        match *self {
            Int(ref n) => fmt::Display::fmt(n, f),
            Real(ref r) => fmt::Display::fmt(r, f),
            Str(ref s) => write!(f, "\"{}\"", s),
            Bool(ref b) => fmt::Display::fmt(b, f),
            Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
}

impl BinOp {
    pub fn to_string(&self) -> &'static str {
        use self::BinOp::*;
        match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Eq => "==",
            Ne => "!=",
            Gt => ">",
            Ge => ">=",
            Lt => "<",
            Le => "<=",
            And => "&",
            Or => "|",
        }
    }
}

#[derive(Debug)]
pub enum UnOp {
    Not,
    Neg,
}

impl UnOp {
    pub fn to_string(&self) -> &'static str {
        use self::UnOp::*;
        match *self {
            Not => "!",
            Neg => "-",
        }
    }
}
