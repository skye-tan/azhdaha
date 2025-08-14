#![allow(clippy::missing_docs_in_private_items)]

use smallvec::SmallVec;

use crate::{
    hir::{self, PrimTyKind, Span, Ty, TyKind},
    mir::{MirCtx, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_rvalue(
        &mut self,
        expr: &'mir hir::Expr,
        bb: &mut BasicBlock,
        stmt_span: Span,
    ) -> Rvalue {
        match &expr.kind {
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb, stmt_span);

                Rvalue::UnaryOp(*un_op, operand)
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb, stmt_span);
                let right_operand = self.lower_to_operand(right_expr, bb, stmt_span);

                Rvalue::BinaryOp(*bin_op, left_operand, right_operand)
            }
            hir::ExprKind::Call(expr, exprs) => {
                let operand = self.lower_to_operand(expr, bb, stmt_span);

                let arguments = exprs
                    .iter()
                    .map(|expr| self.lower_to_operand(expr, bb, stmt_span))
                    .collect();

                Rvalue::Call(operand, arguments)
            }
            hir::ExprKind::Empty => Rvalue::Empty,
            hir::ExprKind::Lit(..)
            | hir::ExprKind::Local(..)
            | hir::ExprKind::Assign(..)
            | hir::ExprKind::Cond(..)
            | hir::ExprKind::Sizeof(..)
            | hir::ExprKind::Field(..) => Rvalue::Use(self.lower_to_operand(expr, bb, stmt_span)),
            kind => panic!("Cannot construct [Rvalue] from: {kind:#?}"),
        }
    }

    pub(crate) fn lower_to_operand(
        &mut self,
        expr: &'mir hir::Expr,
        bb: &mut BasicBlock,
        stmt_span: Span,
    ) -> Operand {
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

                let rvalue = self.lower_to_rvalue(rhs_expr, bb, stmt_span);

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span: stmt_span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb, stmt_span);

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

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), Rvalue::UnaryOp(*un_op, operand)),
                    span: stmt_span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb, stmt_span);
                let right_operand = self.lower_to_operand(right_expr, bb, stmt_span);

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

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        place.clone(),
                        Rvalue::BinaryOp(*bin_op, left_operand, right_operand),
                    ),
                    span: stmt_span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Cond(cond_expr, body_expr, else_expr) => {
                let cond_rvalue = self.lower_to_rvalue(cond_expr, bb, span);

                let cond_local = self.alloc_local(
                    None,
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        is_linear: false,
                        quals: vec![],
                        span: cond_expr.span,
                    },
                    cond_expr.span,
                );

                let cond_place = Place {
                    local: cond_local,
                    projections: vec![],
                    span: cond_expr.span,
                };

                let result_local = self.alloc_local(
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

                let result_place = Place {
                    local: result_local,
                    projections: vec![],
                    span: stmt_span,
                };

                let mut body_bb = self.alloc_bb();
                let body_rvalue = self.lower_to_rvalue(body_expr, &mut body_bb, stmt_span);

                let next_bb = self.alloc_bb();

                self.retrieve_bb(body_bb).statements.push(Statement {
                    kind: StatementKind::Assign(result_place.clone(), body_rvalue),
                    span,
                });

                self.retrieve_bb(body_bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                let mut else_bb = self.alloc_bb();
                let else_rvalue = self.lower_to_rvalue(else_expr, &mut else_bb, stmt_span);

                self.retrieve_bb(else_bb).statements.push(Statement {
                    kind: StatementKind::Assign(result_place.clone(), else_rvalue),
                    span,
                });

                self.retrieve_bb(else_bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(cond_place.clone(), cond_rvalue),
                    span,
                });

                bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::SwitchInt {
                        discr: Operand::Place(cond_place),
                        targets: SwitchTargets {
                            value: SmallVec::from_slice(&[1]),
                            bbs: SmallVec::from_slice(&[body_bb, else_bb]),
                        },
                    },
                    span,
                });

                bb.set(next_bb);

                Operand::Place(result_place)
            }
            // TODO: Inner value must be evaluated and then saved.
            hir::ExprKind::Sizeof(_) => Operand::Const(Const::Sizeof),
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
