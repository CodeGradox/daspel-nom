#[derive(Debug)]
pub enum Expr {
    Lit(Lit),
    BinaryOp(BinOp, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>, Box<Expr>),
    List(Vec<Expr>),
}

#[derive(Debug)]
pub enum Lit {
    Int(i32),
    Real(f32),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnOp {
    Not,
    Neg,
}