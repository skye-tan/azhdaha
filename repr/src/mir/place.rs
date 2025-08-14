#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, PrimTyKind, Span, Ty, TyKind},
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
                let mut place = self.store_in_temp_place(rvalue, bb, stmt_span);

                place.projections.push(PlaceElem::Field(ident.name.clone()));

                place
            }
            hir::ExprKind::Index(expr, index_expr) => {
                let rvalue = self.lower_to_rvalue(expr, bb, stmt_span);
                let mut place = self.store_in_temp_place(rvalue, bb, stmt_span);

                let index_rvalue = self.lower_to_rvalue(index_expr, bb, stmt_span);
                let index_place = self.store_in_temp_place(index_rvalue, bb, stmt_span);

                place.projections.push(PlaceElem::Index(index_place));

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
    ) -> Place {
        let local = self.alloc_local(
            None,
            None,
            &Ty {
                kind: TyKind::PrimTy(PrimTyKind::Int),
                is_linear: false,
                quals: vec![],
                span: stmt_span,
            },
            stmt_span,
        );

        let place = Place {
            local,
            projections: vec![],
            span: stmt_span,
        };

        self.retrieve_bb(*bb).statements.push(Statement {
            kind: StatementKind::Assign(place.clone(), rvalue),
            span: stmt_span,
        });

        place
    }
}
