//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

mod datatypes;

use std::cell::RefCell;

pub use datatypes::*;
use hir_repr::Span;

impl MirCtx {
    pub fn new(f: hir_repr::Fn) -> Self {
        Self {
            result: RefCell::new(Body {
                basic_blocks: vec![],
                local_decls: vec![],
                span: f.body.span.clone(),
            }),
            input: f,
        }
    }

    fn add_local(&self, ty: &hir_repr::Ty, span: Span) {
        self.result.borrow_mut().local_decls.push(LocalDecl { ty: ty.clone(), span });
    }

    pub fn lower(self) -> Body {
        self.add_local(&self.input.sig.ty, self.input.body.span.clone());
        self.lower_expr(&self.input.body);
        self.result.into_inner()
    }

    pub(crate) fn lower_expr(&self, body: &hir_repr::Expr, place: ) {
        match &body.kind {
            hir_repr::ExprKind::Block(block) => {
                for stmt in &block.stmts {
                    match &stmt.kind {
                        hir_repr::StmtKind::Decl(decl_stmt) => todo!(),
                        hir_repr::StmtKind::Expr(expr) => todo!(),
                        hir_repr::StmtKind::Semi(expr) => {
                            self.lower_expr(expr);
                        },
                    }
                }
            },
            hir_repr::ExprKind::Lit(lit) => todo!(),
            hir_repr::ExprKind::Ret(expr) => {

            },
            hir_repr::ExprKind::Path(path) => todo!(),
            hir_repr::ExprKind::Call(expr, exprs) => todo!(),
            hir_repr::ExprKind::Binary(bin_op, expr, expr1) => todo!(),
            hir_repr::ExprKind::Unary(un_op, expr) => todo!(),
            hir_repr::ExprKind::If(expr, expr1, expr2) => todo!(),
            hir_repr::ExprKind::Loop(loop_source, expr) => todo!(),
            hir_repr::ExprKind::Break => todo!(),
            hir_repr::ExprKind::Continue => todo!(),
            hir_repr::ExprKind::Assign(expr, expr1) => todo!(),
            hir_repr::ExprKind::AssignOp(bin_op, expr, expr1) => todo!(),
            hir_repr::ExprKind::Field(expr, ident) => todo!(),
            hir_repr::ExprKind::Index(expr, expr1, span) => todo!(),
            hir_repr::ExprKind::Cast(expr, ty) => todo!(),
            hir_repr::ExprKind::Array(exprs) => todo!(),
            hir_repr::ExprKind::AddrOf(expr) => todo!(),
            hir_repr::ExprKind::Comma(exprs) => todo!(),
            hir_repr::ExprKind::Sizeof(sizeof) => todo!(),
        }
    }
}
