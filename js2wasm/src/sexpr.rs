use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExprValue {
    Atom(String),
    Expr(SExpr),
}

impl Display for SExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExprValue::Atom(s) => write!(f, "{}", s),
            SExprValue::Expr(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SExpr(pub String, pub Vec<SExprValue>);

impl Display for SExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.0)?;

        for v in &self.1 {
            write!(f, " {}", v)?;
        }

        write!(f, ")")
    }
}
