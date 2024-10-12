#[derive(Debug)]
pub enum Expr {
    Const(i32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    Block(Vec<Stmt>),
    Expr(Expr),
}
