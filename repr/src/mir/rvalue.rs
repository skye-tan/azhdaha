#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, Lit, Span},
    mir::{MirCtx, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_rvalue(
        &mut self,
        expr: &hir::Expr,
        bb: &mut BasicBlock,
        stmt_span: Span,
    ) -> Rvalue {
        match &expr.kind {
            hir::ExprKind::Unary(un_op, inner_expr) => match MirUnOp::from_hir(*un_op) {
                MirUnOp::IntUnOp(un_op) => {
                    let operand = self.lower_to_operand(inner_expr, bb, stmt_span);

                    Rvalue::UnaryOp(un_op, operand)
                }
                MirUnOp::AddrOf => {
                    if inner_expr.ty.kind.is_fn() {
                        Rvalue::Cast {
                            value: self.lower_to_operand(inner_expr, bb, stmt_span),
                            from_type: inner_expr.ty.kind.clone(),
                            to_type: expr.ty.kind.clone(),
                        }
                    } else {
                        let place = self.lower_to_place(inner_expr, bb, stmt_span);

                        Rvalue::AddrOf(place)
                    }
                }
                MirUnOp::Deref => Rvalue::Use(self.lower_to_operand(expr, bb, stmt_span)),
            },
            hir::ExprKind::PtrDiff(left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb, stmt_span);
                let right_operand = self.lower_to_operand(right_expr, bb, stmt_span);

                Rvalue::PtrDiff(left_operand, right_operand)
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                match MirBinOp::from_hir(*bin_op) {
                    MirBinOp::IntBinOp(int_bin_op) => {
                        let left_operand = self.lower_to_operand(left_expr, bb, stmt_span);
                        let right_operand = self.lower_to_operand(right_expr, bb, stmt_span);
                        Rvalue::BinaryOp(int_bin_op, left_operand, right_operand)
                    }
                    MirBinOp::ShortCircuitBinOp(short_circuit_bin_op) => {
                        let span = expr.span;
                        let result = self.alloc_temp_place(
                            span,
                            hir::Ty {
                                kind: hir::TyKind::PrimTy(hir::PrimTyKind::Bool),
                                is_linear: false,
                                quals: vec![],
                                span,
                            },
                        );

                        let check_second = self.alloc_bb();
                        let next_block = self.alloc_bb();
                        let fail_path = self.alloc_bb();
                        let happy_path = self.alloc_bb();

                        let first_cond = self.lower_to_operand(left_expr, bb, stmt_span);

                        let first_cond_targets = match short_circuit_bin_op {
                            ShortCircuitBinOp::And => [check_second, fail_path],
                            ShortCircuitBinOp::Or => [happy_path, check_second],
                        };

                        self.retrieve_bb(*bb).terminator = Some(Terminator {
                            kind: TerminatorKind::SwitchInt {
                                discr: first_cond,
                                targets: first_cond_targets,
                            },
                            span,
                        });

                        bb.set(check_second);

                        let second_cond = self.lower_to_operand(right_expr, bb, stmt_span);

                        self.retrieve_bb(*bb).terminator = Some(Terminator {
                            kind: TerminatorKind::SwitchInt {
                                discr: second_cond,
                                targets: [happy_path, fail_path],
                            },
                            span,
                        });

                        self.retrieve_bb(fail_path).statements.push(Statement {
                            kind: StatementKind::Assign(
                                result.clone(),
                                Rvalue::Use(Operand::Const(Const::Lit(Lit {
                                    kind: hir::LitKind::Int(0),
                                    span,
                                }))),
                            ),
                            span,
                        });
                        self.retrieve_bb(happy_path).statements.push(Statement {
                            kind: StatementKind::Assign(
                                result.clone(),
                                Rvalue::Use(Operand::Const(Const::Lit(Lit {
                                    kind: hir::LitKind::Int(1),
                                    span,
                                }))),
                            ),
                            span,
                        });
                        self.retrieve_bb(fail_path).terminator = Some(Terminator {
                            kind: TerminatorKind::Goto { bb: next_block },
                            span,
                        });
                        self.retrieve_bb(happy_path).terminator = Some(Terminator {
                            kind: TerminatorKind::Goto { bb: next_block },
                            span,
                        });
                        bb.set(next_block);

                        Rvalue::Use(Operand::Place(result))
                    }
                }
            }
            hir::ExprKind::Call(expr, exprs) => {
                let operand = self.lower_to_operand(expr, bb, stmt_span);

                let arguments = exprs
                    .iter()
                    .map(|expr| self.lower_to_operand(expr, bb, stmt_span))
                    .collect();

                Rvalue::Call(operand, arguments)
            }
            hir::ExprKind::Cast(inner_expr) => {
                if let hir::ExprKind::InitializerList(tree) = &inner_expr.kind {
                    let tree = self.lower_to_initializer_tree(tree, bb);
                    Rvalue::CompoundInitializing(expr.ty.kind.clone(), tree)
                } else {
                    let operand = self.lower_to_operand(inner_expr, bb, stmt_span);

                    Rvalue::Cast {
                        value: operand,
                        from_type: inner_expr.ty.kind.clone(),
                        to_type: expr.ty.kind.clone(),
                    }
                }
            }
            hir::ExprKind::Comma(exprs) => {
                let (first_expr, exprs) = exprs.split_first().unwrap();

                let first_place = self.lower_to_operand(first_expr, bb, stmt_span);

                for expr in exprs {
                    _ = self.lower_to_operand(expr, bb, stmt_span);
                }

                Rvalue::Use(first_place)
            }
            hir::ExprKind::InitializerList(_) => {
                panic!("Using initializer lists as expression is invalid.");
            }
            hir::ExprKind::Empty => Rvalue::Empty,
            hir::ExprKind::VaArg(va_list, arg_ty) => Rvalue::VaArg(
                self.lower_to_operand(va_list, bb, stmt_span),
                arg_ty.clone(),
            ),
            hir::ExprKind::GnuBlock(_)
            | hir::ExprKind::PtrOffset(..)
            | hir::ExprKind::Lit(..)
            | hir::ExprKind::Local(..)
            | hir::ExprKind::Assign(..)
            | hir::ExprKind::AssignWithBinOp(..)
            | hir::ExprKind::AssignPtrOffset(..)
            | hir::ExprKind::Cond(..)
            | hir::ExprKind::Sizeof(..)
            | hir::ExprKind::OffsetOf
            | hir::ExprKind::Field(..) => Rvalue::Use(self.lower_to_operand(expr, bb, stmt_span)),
        }
    }
}
