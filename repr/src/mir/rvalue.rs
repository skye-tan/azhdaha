#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self, Span},
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
            hir::ExprKind::Unary(un_op, inner_expr) => match MirUnOp::from_hir(*un_op) {
                MirUnOp::IntUnOp(un_op) => {
                    let operand = self.lower_to_operand(inner_expr, bb, stmt_span);

                    Rvalue::UnaryOp(un_op, operand)
                }
                MirUnOp::AddrOf => {
                    let place = self.lower_to_place(inner_expr, bb, stmt_span);

                    Rvalue::AddrOf(place)
                }
                MirUnOp::Deref => Rvalue::Use(self.lower_to_operand(expr, bb, stmt_span)),
            },
            hir::ExprKind::PtrDiff(left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb, stmt_span);
                let right_operand = self.lower_to_operand(right_expr, bb, stmt_span);

                Rvalue::PtrDiff(left_operand, right_operand)
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb, stmt_span);
                let right_operand = self.lower_to_operand(right_expr, bb, stmt_span);

                match MirBinOp::from_hir(*bin_op) {
                    MirBinOp::IntBinOp(int_bin_op) => {
                        Rvalue::BinaryOp(int_bin_op, left_operand, right_operand)
                    }
                    MirBinOp::ShortCircuitBinOp(short_circuit_bin_op) => {
                        // TODO: make these actually short circuit
                        let op = match short_circuit_bin_op {
                            ShortCircuitBinOp::And => IntBinOp::BitAnd,
                            ShortCircuitBinOp::Or => IntBinOp::BitOr,
                        };
                        Rvalue::BinaryOp(op, left_operand, right_operand)
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
                if let hir::ExprKind::Array(array) = &inner_expr.kind {
                    match &expr.ty.kind {
                        hir::TyKind::Struct(idx) => {
                            let mut ops = vec![];
                            for elem in array {
                                ops.push(self.lower_to_operand(elem, bb, stmt_span));
                            }
                            Rvalue::StructInitializing(*idx, ops)
                        }
                        _ => panic!("Invalid cast from initializer list to {}", expr.ty),
                    }
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
            hir::ExprKind::Array(exprs) => {
                let mut operands = vec![];

                for expr in exprs {
                    operands.push(self.lower_to_operand(expr, bb, stmt_span));
                }

                Rvalue::List(operands)
            }
            hir::ExprKind::Empty => Rvalue::Empty,
            hir::ExprKind::GnuBlock(_)
            | hir::ExprKind::PtrOffset(..)
            | hir::ExprKind::Lit(..)
            | hir::ExprKind::Local(..)
            | hir::ExprKind::Assign(..)
            | hir::ExprKind::Cond(..)
            | hir::ExprKind::Sizeof(..)
            | hir::ExprKind::Field(..) => Rvalue::Use(self.lower_to_operand(expr, bb, stmt_span)),
        }
    }
}
