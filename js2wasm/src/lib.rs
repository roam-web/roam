mod sexpr;
pub use sexpr::{SExpr, SExprValue};

use color_eyre::Result;
use oxc::{allocator::Allocator, ast::AstKind, parser::Parser, span::SourceType};
use oxc_semantic::{AstNode, SemanticBuilder};

fn compile_node(node: &AstNode) -> Result<SExprValue> {
    match node.kind() {
        AstKind::ExpressionStatement(expr) => match &expr.expression {
            _ => Ok(SExprValue::Atom("TODO".to_string())),
        },
        _ => Ok(SExprValue::Atom("TODO".to_string())),
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

    let sem = SemanticBuilder::new(&src, SourceType::default())
        .with_trivias(res.trivias)
        .build(&res.program);

    let compiled = sem
        .semantic
        .nodes()
        .iter()
        .map(|node| compile_node(node))
        .collect::<Result<Vec<_>>>()?;

    let mainfn = SExpr("func".to_string(), compiled);
    Ok(SExpr("module".to_string(), vec![SExprValue::Expr(mainfn)]))
}
