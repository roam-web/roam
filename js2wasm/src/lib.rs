mod sexpr;
pub use sexpr::{SExpr, SExprValue};

pub(crate) mod decl;

use color_eyre::Result;
use oxc::{allocator::Allocator, ast::ast::Statement as Stmt, parser::Parser, span::SourceType};

pub(crate) fn compile_node(stmt: &Stmt) -> Result<Vec<SExprValue>> {
    match stmt {
        Stmt::Declaration(decl) => decl::compile_decl(decl),
        _ => todo!("{:#?}", stmt),
    }
}

pub fn compile<S: ToString>(src: S) -> Result<SExpr> {
    let allocator = Allocator::default();
    let src = src.to_string();
    let res = Parser::new(&allocator, &src, SourceType::default()).parse();

    if !res.errors.is_empty() {
        return Err(color_eyre::eyre::eyre!(res
            .errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")));
    }

    let compiled = res
        .program
        .body
        .iter()
        .map(|node| compile_node(node))
        .collect::<Result<Vec<_>>>()?
        .concat();

    let startfn = SExpr("func".to_string(), compiled);
    let memexport = SExpr(
        "export".to_string(),
        vec![
            SExprValue::Atom("\"memory\"".to_string()),
            SExprValue::Expr(SExpr(
                "memory".to_string(),
                vec![SExprValue::Atom("0".to_string())],
            )),
        ],
    );
    let startexport = SExpr(
        "export".to_string(),
        vec![
            SExprValue::Atom("\"_start\"".to_string()),
            SExprValue::Expr(SExpr(
                "func".to_string(),
                vec![SExprValue::Atom("0".to_string())],
            )),
        ],
    );
    Ok(SExpr(
        "module".to_string(),
        vec![
            SExprValue::Expr(startfn),
            SExprValue::Expr(memexport),
            SExprValue::Expr(startexport),
        ],
    ))
}
