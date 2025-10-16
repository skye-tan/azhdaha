#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, Span, Ty},
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
            hir::ExprKind::Local(symbol) => {
                let local = self.local_map.get(symbol).unwrap();

                Place {
                    local: *local,
                    projections: vec![],
                    span,
                }
            }
            hir::ExprKind::Field(expr, ident) => {
                let rvalue = self.lower_to_rvalue(expr, bb, stmt_span);
                let mut place = self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone());

                place.projections.push(PlaceElem::Field(ident.name.clone()));

                place
            }
            hir::ExprKind::Index(expr, index_expr) => {
                let rvalue = self.lower_to_rvalue(expr, bb, stmt_span);
                let mut place = self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone());

                let index_rvalue = self.lower_to_rvalue(index_expr, bb, stmt_span);
                let index_place =
                    self.store_in_temp_place(index_rvalue, bb, stmt_span, index_expr.ty.clone());

                place.projections.push(PlaceElem::Index(index_place));

                place
            }
            hir::ExprKind::Unary(hir::UnOp::Deref, expr) => {
                let rvalue = self.lower_to_rvalue(expr, bb, stmt_span);
                let mut place = self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone());

                place.projections.push(PlaceElem::Deref);

                place
            }
            kind => panic!("Cannot construct [Place] from: {kind:#?}"),
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
