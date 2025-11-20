use crate::{
    hir::{self, ExprKind, TyKind},
    mir::{BasicBlock, MirCtx, Operand},
};

#[derive(Debug, Clone)]
pub enum InitializerTree {
    Middle { children: Vec<InitializerTree> },
    Leaf(Operand),
    Zeroed,
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
            if let Some(designator) = &item.designator {
                match designator {
                    hir::Designator::Subscript { value } => {
                        let value = *value as usize;
                        while children.len() < value {
                            children.push(InitializerTree::Zeroed);
                        }
                    }
                }
            }
            children.push(self.lower_to_initializer_tree(expected_ty, &item.value, bb));
        }
        InitializerTree::Middle { children }
    }
}
