use crate::{
    hir,
    mir::{BasicBlock, MirCtx, Operand},
};

#[derive(Debug, Clone)]
pub enum MirInitializerTree {
    Middle { children: Vec<MirInitializerTree> },
    Leaf(Operand),
    Zeroed,
}

impl<'mir> MirCtx<'mir> {
    /// Convert a hir expression to an initializer tree.
    pub(crate) fn lower_to_initializer_tree(
        &mut self,
        tree: &hir::InitializerTree,
        bb: &mut BasicBlock,
    ) -> MirInitializerTree {
        match tree {
            hir::InitializerTree::Middle { children } => MirInitializerTree::Middle {
                children: children
                    .iter()
                    .map(|child| self.lower_to_initializer_tree(child, bb))
                    .collect(),
            },
            hir::InitializerTree::Leaf(expr) => {
                MirInitializerTree::Leaf(self.lower_to_operand(expr, bb, expr.span))
            }
            hir::InitializerTree::Zeroed => MirInitializerTree::Zeroed,
        }
    }
}
