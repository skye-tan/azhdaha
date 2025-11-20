#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, Span, UnOp},
    mir::{MirCtx, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_operand(
        &mut self,
        expr: &hir::Expr,
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
            hir::ExprKind::AssignWithBinOp(
                lhs_expr,
                rhs_expr,
                binop,
                binop_ty,
                return_semantics,
            ) => {
                let binop = match MirBinOp::from_hir(*binop) {
                    MirBinOp::IntBinOp(int_bin_op) => int_bin_op,
                    MirBinOp::ShortCircuitBinOp(_) => {
                        panic!("Invalid binop for assignment.")
                    }
                };

                let lhs = self.lower_to_place(lhs_expr, bb, stmt_span);

                let rhs = self.lower_to_operand(rhs_expr, bb, stmt_span);

                let temp = self.alloc_temp_place(stmt_span, binop_ty.clone());
                let old_lhs = self.alloc_temp_place(stmt_span, binop_ty.clone());

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        old_lhs.clone(),
                        Rvalue::Cast {
                            value: Operand::Place(lhs.clone()),
                            from_type: expr.ty.kind.clone(),
                            to_type: binop_ty.kind.clone(),
                        },
                    ),
                    span: stmt_span,
                });

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        temp.clone(),
                        Rvalue::BinaryOp(binop, Operand::Place(old_lhs.clone()), rhs),
                    ),
                    span: stmt_span,
                });

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        lhs.clone(),
                        Rvalue::Cast {
                            value: Operand::Place(temp),
                            from_type: binop_ty.kind.clone(),
                            to_type: expr.ty.kind.clone(),
                        },
                    ),
                    span: stmt_span,
                });

                match return_semantics {
                    hir::ReturnSemantic::AfterAssign => Operand::Place(lhs),
                    hir::ReturnSemantic::BeforeAssign => Operand::Place(old_lhs),
                }
            }
            hir::ExprKind::AssignPtrOffset(lhs_expr, rhs_expr, return_semantics) => {
                let lhs = self.lower_to_place(lhs_expr, bb, stmt_span);

                let rhs = self.lower_to_place(rhs_expr, bb, stmt_span);

                let mut offsetted_lhs = lhs.clone();
                offsetted_lhs.projections.push(PlaceElem::Index(rhs));

                let old_lhs = self.alloc_temp_place(stmt_span, expr.ty.clone());

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        old_lhs.clone(),
                        Rvalue::Use(Operand::Place(lhs.clone())),
                    ),
                    span: stmt_span,
                });

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        lhs.clone(),
                        Rvalue::Use(Operand::Place(offsetted_lhs)),
                    ),
                    span: stmt_span,
                });

                match return_semantics {
                    hir::ReturnSemantic::AfterAssign => Operand::Place(lhs),
                    hir::ReturnSemantic::BeforeAssign => Operand::Place(old_lhs),
                }
            }
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr, bb, stmt_span);

                let rvalue = self.lower_to_rvalue(rhs_expr, bb, stmt_span);

                let bb_data = self.retrieve_bb(*bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span: stmt_span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Unary(un_op, _) => {
                if *un_op == UnOp::Deref {
                    Operand::Place(self.lower_to_place(expr, bb, stmt_span))
                } else {
                    let rvalue = self.lower_to_rvalue(expr, bb, stmt_span);

                    let place = self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone());

                    Operand::Place(place)
                }
            }
            hir::ExprKind::Binary(..) => {
                let rvalue = self.lower_to_rvalue(expr, bb, span);
                let place = self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone());

                Operand::Place(place)
            }
            hir::ExprKind::Cond(cond_expr, body_expr, else_expr) => {
                let cond_rvalue = self.lower_to_rvalue(cond_expr, bb, span);
                let cond_place =
                    self.store_in_temp_place(cond_rvalue, bb, stmt_span, cond_expr.ty.clone());

                let mut body_bb = self.alloc_bb();
                let body_rvalue = self.lower_to_rvalue(body_expr, &mut body_bb, stmt_span);

                let next_bb = self.alloc_bb();

                let mut else_bb = self.alloc_bb();
                let else_rvalue = self.lower_to_rvalue(else_expr, &mut else_bb, stmt_span);

                let result_place = self.alloc_temp_place(stmt_span, expr.ty.clone());

                self.retrieve_bb(body_bb).statements.push(Statement {
                    kind: StatementKind::Assign(result_place.clone(), body_rvalue),
                    span,
                });
                self.retrieve_bb(body_bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                self.retrieve_bb(else_bb).statements.push(Statement {
                    kind: StatementKind::Assign(result_place.clone(), else_rvalue),
                    span,
                });
                self.retrieve_bb(else_bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                self.retrieve_bb(*bb).terminator = Some(Terminator {
                    kind: TerminatorKind::SwitchInt {
                        discr: Operand::Place(cond_place),
                        targets: [body_bb, else_bb],
                    },
                    span,
                });

                bb.set(next_bb);

                Operand::Place(result_place)
            }
            hir::ExprKind::Call(..) | hir::ExprKind::Cast(..) | hir::ExprKind::PtrDiff(..) => {
                let rvalue = self.lower_to_rvalue(expr, bb, span);
                let place = self.store_in_temp_place(rvalue, bb, stmt_span, expr.ty.clone());

                Operand::Place(place)
            }
            // TODO: Inner value must be evaluated and then saved.
            hir::ExprKind::Sizeof(sizeof) => Operand::Const(Const::Sizeof(match &sizeof.kind {
                hir::SizeofKind::Ty(ty) => ty.clone(),
                hir::SizeofKind::Expr(expr) => expr.ty.clone(),
            })),

            hir::ExprKind::PtrOffset(..)
            | hir::ExprKind::Field(..)
            | hir::ExprKind::GnuBlock(_) => {
                Operand::Place(self.lower_to_place(expr, bb, stmt_span))
            }
            kind => panic!("Cannot construct [Operand] from: {kind:#?}"),
        }
    }
}
