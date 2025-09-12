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
            hir::ExprKind::Cast(expr, ty_kind) => {
                let operand = self.lower_to_operand(expr, bb, stmt_span);

                Rvalue::Cast(operand, ty_kind.clone())
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
            hir::ExprKind::Lit(..)
            | hir::ExprKind::Local(..)
            | hir::ExprKind::Assign(..)
            | hir::ExprKind::Cond(..)
            | hir::ExprKind::Sizeof(..)
            | hir::ExprKind::Field(..)
            | hir::ExprKind::Index(..) => Rvalue::Use(self.lower_to_operand(expr, bb, stmt_span)),
        }
    }
}
