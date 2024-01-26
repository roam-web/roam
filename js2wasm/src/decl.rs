use crate::{SExpr, SExprValue};

use color_eyre::Result;
use oxc::ast::ast::{
    BindingPatternKind as BindKind, Declaration as Decl, VariableDeclarationKind as VarDeclKind,
};

pub fn compile_decl(decl: &Decl) -> Result<Vec<SExprValue>> {
    match decl {
        Decl::VariableDeclaration(decl) => {
            let mut vars = match decl.kind {
                VarDeclKind::Var => SExpr(
                    "global".to_string(),
                    vec![SExprValue::Atom("sometype".to_string())],
                ),
                VarDeclKind::Let => SExpr(
                    "local".to_string(),
                    vec![SExprValue::Atom("sometype".to_string())],
                ),
                VarDeclKind::Const => SExpr(
                    "const".to_string(),
                    vec![SExprValue::Atom("sometype".to_string())],
                ),
            };

            let mut inits = vec![];
            for decl in &decl.declarations {
                match &decl.id.kind {
                    BindKind::BindingIdentifier(id) => {
                        vars.1.push(SExprValue::Atom(format!("$__js_{}", id.name)))
                    }
                    _ => unimplemented!("{:#?}", decl),
                }

                match &decl.init {
                    Some(_) => inits.push(unimplemented!()),
                    None => {}
                }
            }

            Ok(vec![vec![SExprValue::Expr(vars)], inits].concat())
        }
        _ => unimplemented!("{:#?}", decl),
    }
}
