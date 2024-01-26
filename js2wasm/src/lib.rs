use color_eyre::Result;
use oxc::{
    allocator::Allocator,
    ast::{
        ast::{Expression, Statement as Stmt},
        AstKind,
    },
    parser::Parser,
    span::SourceType,
};
use oxc_semantic::SemanticBuilder;

pub fn compile<S: ToString>(src: S) -> Result<String> {
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

    let mut ret = "".to_string();
    let sem = SemanticBuilder::new(&src, SourceType::default())
        .with_trivias(res.trivias)
        .build(&res.program);

    for node in sem.semantic.nodes().iter() {
        match node.kind() {
            AstKind::ExpressionStatement(expr) => match &expr.expression {
                Expression::BooleanLiteral(expr) => {}
                _ => {}
            },
            _ => {}
        }
    }

    Ok(ret)
}
