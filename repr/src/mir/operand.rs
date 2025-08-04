#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, PrimTyKind, Ty, TyKind},
    mir::{MirCtx, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_rvalue(&mut self, expr: &'mir hir::Expr, bb: BasicBlock) -> Rvalue {
        match &expr.kind {
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb);

                Rvalue::UnaryOp(*un_op, operand)
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb);
                let right_operand = self.lower_to_operand(right_expr, bb);

                Rvalue::BinaryOp(*bin_op, left_operand, right_operand)
            }
            hir::ExprKind::Call(expr, exprs) => {
                let operand = self.lower_to_operand(expr, bb);

                let arguments = exprs
                    .iter()
                    .map(|expr| self.lower_to_operand(expr, bb))
                    .collect();

                Rvalue::Call(operand, arguments)
            }
            hir::ExprKind::Empty => Rvalue::Empty,
            hir::ExprKind::Lit(..)
            | hir::ExprKind::Local(..)
            | hir::ExprKind::Assign(..)
            | hir::ExprKind::Field(..) => Rvalue::Use(self.lower_to_operand(expr, bb)),
            kind => panic!("Cannot construct [Rvalue] from: {kind:#?}"),
        }
    }

    pub(crate) fn lower_to_operand(&mut self, expr: &'mir hir::Expr, bb: BasicBlock) -> Operand {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Lit(lit) => Operand::Const(Const::Lit(lit.clone())),
            hir::ExprKind::Local(symbol) => match self.local_map.get(symbol) {
                Some(local) => Operand::Place(Place {
                    local: *local,
                    projections: vec![],
                    span,
                }),
                None => Operand::Const(Const::Symbol(*symbol)),
            },
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr, bb);

                let bb_data = self.retrieve_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb);

                let local = self.alloc_local(
                    None,
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        is_linear: false,
                        quals: vec![],
                        span,
                    },
                    span,
                );

                let place = Place {
                    local,
                    projections: vec![],
                    span,
                };

                let bb_data = self.retrieve_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), Rvalue::UnaryOp(*un_op, operand)),
                    span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb);
                let right_operand = self.lower_to_operand(right_expr, bb);

                let local = self.alloc_local(
                    None,
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        is_linear: false,
                        quals: vec![],
                        span,
                    },
                    span,
                );

                let place = Place {
                    local,
                    projections: vec![],
                    span,
                };

                let bb_data = self.retrieve_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        place.clone(),
                        Rvalue::BinaryOp(*bin_op, left_operand, right_operand),
                    ),
                    span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Field(..) => Operand::Place(self.lower_to_place(expr)),
            kind => panic!("Cannot construct [Operand] from: {kind:#?}"),
        }
    }

    pub(crate) fn lower_to_place(&mut self, expr: &'mir hir::Expr) -> Place {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Local(res) => {
                let local = self.local_map.get(res).unwrap();

                Place {
                    local: *local,
                    projections: vec![],
                    span,
                }
            }
            hir::ExprKind::Field(expr, ident) => {
                let mut place = self.lower_to_place(expr);

                place.projections.push(PlaceElem::Field(ident.name.clone()));

                place
            }
            kind => panic!("Cannot construct [Place] from: {kind:#?}"),
        }
    }
}
