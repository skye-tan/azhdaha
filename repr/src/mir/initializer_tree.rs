use crate::{
    hir::{self, ExprKind, LitKind, Ty, TyKind},
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
        expr: &hir::Expr,
        bb: &mut BasicBlock,
    ) -> InitializerTree {
        let list = match &expr.kind {
            ExprKind::InitializerList(list) => list,
            ExprKind::Lit(lit) => {
                if let LitKind::Str(string) = &lit.kind {
                    let init_expr = initializer_list_from_string(
                        string,
                        Ty {
                            kind: expected_ty.clone(),
                            is_linear: false,
                            quals: vec![],
                            span: expr.span,
                        },
                        expr.span,
                    );
                    return self.lower_to_initializer_tree(expected_ty, &init_expr, bb);
                } else {
                    let op = self.lower_to_operand(expr, bb, expr.span);
                    return InitializerTree::Leaf(op);
                }
            }
            _ => {
                let op = self.lower_to_operand(expr, bb, expr.span);
                return InitializerTree::Leaf(op);
            }
        };
        let mut children = vec![];
        for item in list {
            if let Some(designator) = &item.designator {
                match designator {
                    hir::Designator::Subscript { value } => {
                        let value = *value as usize;
                        while children.len() <= value {
                            children.push(InitializerTree::Zeroed);
                        }
                        children[value] =
                            self.lower_to_initializer_tree(expected_ty, &item.value, bb);
                    }
                }
            } else {
                children.push(self.lower_to_initializer_tree(expected_ty, &item.value, bb));
            }
        }
        InitializerTree::Middle { children }
    }
}

pub(crate) fn initializer_list_from_string(
    string: &str,
    ty: hir::Ty,
    span: hir::Span,
) -> hir::Expr {
    use hir::{Expr, ExprKind, InitializerItem, Lit, LitKind};
    Expr {
        kind: ExprKind::InitializerList(
            string
                .chars()
                .map(|ch| InitializerItem {
                    designator: None,
                    value: Expr {
                        kind: ExprKind::Lit(Lit {
                            kind: LitKind::Char(ch),
                            span,
                        }),
                        ty: ty.clone(),
                        span,
                    },
                })
                .collect(),
        ),
        ty,
        span,
    }
}
