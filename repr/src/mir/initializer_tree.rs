use crate::{
    hir::{
        self, Designator, ExprKind, LitKind, Ty, TyKind,
        resolver::{CompoundTypeData, Resolver},
    },
    mir::{BasicBlock, MirCtx, Operand},
};

#[derive(Debug, Clone)]
pub enum InitializerTree {
    Middle { children: Vec<InitializerTree> },
    Leaf(Operand),
    Zeroed,
}
impl InitializerTree {
    /// Returns the children of this initializer list, break it if it was empty.
    fn children(&mut self) -> &mut Vec<InitializerTree> {
        match self {
            InitializerTree::Middle { children } => children,
            InitializerTree::Leaf(_) => panic!("Leaf does not have children."),
            InitializerTree::Zeroed => {
                *self = InitializerTree::Middle { children: vec![] };
                let InitializerTree::Middle { children } = self else {
                    unreachable!();
                };
                children
            }
        }
    }
}

/// The state of the initializer list.
#[derive(Debug)]
struct InitializerCursor {
    /// Type of the root element.
    base_ty: TyKind,
    /// Index and type in each level.
    stack: Vec<(usize, TyKind)>,
}

impl InitializerCursor {
    /// Create a cursor from a designator list.
    fn from_designators(
        designators: &[Designator],
        base_ty: &TyKind,
        ttr: &Resolver<CompoundTypeData>,
    ) -> Self {
        let mut base_ty = base_ty;
        let mut result = InitializerCursor {
            base_ty: base_ty.clone(),
            stack: vec![],
        };
        for designator in designators {
            match designator {
                Designator::Subscript { value } => {
                    match base_ty {
                        TyKind::Array { kind, size: _ } => {
                            base_ty = kind;
                        }
                        _ => panic!("Invalid array designator for type {base_ty:?}"),
                    }
                    result.stack.push((*value as usize, base_ty.clone()));
                }
                Designator::Field { name } => {
                    let fields = base_ty.fields(ttr).unwrap();
                    let addr = &fields.by_name[name];
                    for &elem in addr {
                        let fields = base_ty.fields(ttr).unwrap();
                        base_ty = &fields.by_index[elem].kind;
                        result.stack.push((elem, base_ty.clone()));
                    }
                }
            }
        }
        result
    }

    /// Returns the current type under the cursor.
    fn ty(&self) -> &TyKind {
        match self.stack.last() {
            Some((_, ty)) => ty,
            None => &self.base_ty,
        }
    }

    /// Insert some expression in this place.
    fn insert_to_tree(
        &self,
        result: &mut InitializerTree,
        value: &hir::Expr,
        ctx: &mut MirCtx,
        bb: &mut BasicBlock,
    ) {
        if self.stack.is_empty() {
            return;
        }
        let mut current = result;
        for (item, _) in &self.stack {
            let children = current.children();
            while children.len() <= *item {
                children.push(InitializerTree::Zeroed);
            }
            current = &mut children[*item];
        }
        *current = ctx.lower_to_initializer_tree(self.ty(), value, bb)
    }

    /// Go down to the first non compound type to initialize.
    fn go_through_primitive(&mut self, ttr: &Resolver<CompoundTypeData>) {
        if self.stack.is_empty() {
            return;
        }
        match self.ty() {
            ty @ (TyKind::Struct(_) | TyKind::Union(_)) => {
                let fields = ty.fields(ttr).unwrap();
                self.stack.push((0, fields.by_index[0].kind.clone()));
                self.go_through_primitive(ttr);
            }
            TyKind::Array { kind, size: _ } => {
                self.stack.push((0, (**kind).clone()));
                self.go_through_primitive(ttr);
            }
            _ => (),
        }
    }

    /// Move cursor to the next element, clear it if the struct is finished.
    fn go_next(&mut self, ttr: &Resolver<CompoundTypeData>) {
        if self.stack.is_empty() {
            return;
        }
        let mut last = self.stack.pop().unwrap();
        match self.ty() {
            ty @ TyKind::Struct(_) => {
                let fields = ty.fields(ttr).unwrap();
                last.0 += 1;
                match fields.by_index.get(last.0) {
                    Some(field) => {
                        last.1 = field.kind.clone();
                        self.stack.push(last);
                    }
                    None => {
                        self.go_next(ttr);
                    }
                }
            }
            TyKind::Union(_) => {
                self.go_next(ttr);
            }
            TyKind::Array { .. } => {
                last.0 += 1;
                self.stack.push(last);
            }
            _ => panic!("Being in child of a primitive is impossible."),
        }
    }

    /// Create a new cursor to the base ty.
    fn to_first(ttr: &Resolver<CompoundTypeData>, base_ty: &TyKind) -> Self {
        match base_ty {
            ty @ (TyKind::Struct(_) | TyKind::Union(_)) => {
                let fields = ty.fields(ttr).unwrap();
                Self {
                    base_ty: base_ty.clone(),
                    stack: vec![(0, fields.by_index[0].kind.clone())],
                }
            }
            TyKind::Array { kind, size: _ } => Self {
                base_ty: base_ty.clone(),
                stack: vec![(0, (**kind).clone())],
            },
            _ => panic!("Invalid type {base_ty:?} for initializer list."),
        }
    }
}

impl<'mir> MirCtx<'mir> {
    /// Convert a hir expression to an initializer tree.
    pub(crate) fn lower_to_initializer_tree(
        &mut self,
        expected_ty: &TyKind,
        expr: &hir::Expr,
        bb: &mut BasicBlock,
    ) -> InitializerTree {
        let ttr = self.body.type_tag_resolver;
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
        let mut result = InitializerTree::Middle { children: vec![] };
        let mut cursor = InitializerCursor::to_first(ttr, expected_ty);
        for item in list {
            if let Some(designators) = &item.designators {
                cursor = InitializerCursor::from_designators(designators, expected_ty, ttr);
            }
            if !matches!(item.value.kind, ExprKind::InitializerList(_)) {
                cursor.go_through_primitive(ttr);
            }
            cursor.insert_to_tree(&mut result, &item.value, self, bb);
            cursor.go_next(ttr);
        }
        result
    }
}

/// Construct an initializer list from a string literal.
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
                    designators: None,
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
