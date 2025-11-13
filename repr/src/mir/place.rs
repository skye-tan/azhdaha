#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, Span, StmtKind, Ty},
    mir::{MirCtx, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_place(
        &mut self,
        expr: &'mir hir::Expr,
        bb: &mut BasicBlock,
        stmt_span: Span,
    ) -> Place {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Local(symbol) => match self.local_map.get(symbol) {
                Some(local) => Place {
                    local: *local,
                    projections: vec![],
                    span,
                },
                None => {
                    // statics
                    let mut addr_place = self.store_in_temp_place(
                        Rvalue::AddrOfStatic(*symbol),
                        bb,
                        stmt_span,
                        Ty {
                            kind: hir::TyKind::Ptr {
                                kind: Box::new(expr.ty.kind.clone()),
                                quals: vec![],
                            },
                            is_linear: false,
                            quals: vec![],
                            span,
                        },
                    );
                    addr_place.projections.push(PlaceElem::Deref);
                    addr_place
                }
            },
            hir::ExprKind::Field(expr, ident) => {
                let mut place = self.lower_to_place(expr, bb, stmt_span);

                place.projections.push(PlaceElem::Field(ident.name.clone()));

                place
            }
            hir::ExprKind::PtrOffset(expr, index_expr) => {
                let mut place = self.lower_to_place(expr, bb, stmt_span);

                let index_rvalue = self.lower_to_rvalue(index_expr, bb, stmt_span);
                let index_place =
                    self.store_in_temp_place(index_rvalue, bb, stmt_span, index_expr.ty.clone());

                place.projections.push(PlaceElem::Index(index_place));

                place
            }
            hir::ExprKind::Unary(hir::UnOp::Deref, expr) => {
                let mut place = self.lower_to_place(expr, bb, stmt_span);

                place.projections.push(PlaceElem::Deref);

                place
            }
            hir::ExprKind::GnuBlock(block) => {
                let Some((last, base)) = block.stmts.split_last() else {
                    panic!("Invalid gnu block expression");
                };
                let StmtKind::Expr(last) = &last.kind else {
                    panic!("Invalid gnu block last statement");
                };

                let saved_symbol_resolver = self.body.symbol_resolver;
                self.body.symbol_resolver = &block.symbol_resolver;

                for stmt in base {
                    self.lower_to_bb(stmt, bb);
                }

                let place = self.lower_to_place(last, bb, stmt_span);

                self.body.symbol_resolver = saved_symbol_resolver;

                place
            }
            _ => {
                let rvalue = self.lower_to_rvalue(expr, bb, stmt_span);
                self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone())
            }
        }
    }

    pub(crate) fn store_in_temp_place(
        &mut self,
        rvalue: Rvalue,
        bb: &mut BasicBlock,
        stmt_span: Span,
        ty: Ty,
    ) -> Place {
        let place = self.alloc_temp_place(stmt_span, ty);

        self.retrieve_bb(*bb).statements.push(Statement {
            kind: StatementKind::Assign(place.clone(), rvalue),
            span: stmt_span,
        });

        place
    }

    pub(crate) fn alloc_temp_place(&mut self, stmt_span: Span, ty: Ty) -> Place {
        let local = self.alloc_temp_local(stmt_span, ty);

        Place {
            local,
            projections: vec![],
            span: stmt_span,
        }
    }
}
