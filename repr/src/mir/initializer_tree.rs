use crate::{
    hir::{self, ExprKind, TyKind},
    mir::{BasicBlock, MirCtx, Operand},
};

#[derive(Debug, Clone)]
pub enum InitializerTree {
    Middle { children: Vec<InitializerTree> },
    Leaf(Operand),
}

impl InitializerTree {}

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_initializer_tree(
        &mut self,
        expected_ty: &TyKind,
        expr: &'mir hir::Expr,
        bb: &mut BasicBlock,
    ) -> InitializerTree {
        let ExprKind::InitializerList(list) = &expr.kind else {
            let op = self.lower_to_operand(expr, bb, expr.span);
            return InitializerTree::Leaf(op);
        };
        let mut children = vec![];
        for item in list {
            children.push(self.lower_to_initializer_tree(expected_ty, item, bb));
        }
        InitializerTree::Middle { children }
    }
}
