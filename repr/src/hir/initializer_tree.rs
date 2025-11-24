use azhdaha_errors::Span;

use crate::hir::{
    self, Designator, DesignatorKind, Expr, ExprKind, ExprOrList, HirCtx, LitKind, Ty, TyKind,
    resolver::{CompoundTypeData, Resolver},
};

#[derive(Debug)]
pub enum InitializerTree {
    Middle { children: Vec<InitializerTree> },
    Leaf(Expr),
    Zeroed,
}
impl InitializerTree {
    /// Returns the children of this initializer list, break it if it was empty.
    ///
    /// # Panics
    /// Panics if the node is a leaf.
    pub fn children(&mut self) -> &mut Vec<InitializerTree> {
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
    /// The span of the initializer list.
    span: Span,
}

impl InitializerCursor {
    /// Create a cursor from a designator list.
    fn from_designators(
        designators: &[Designator],
        base_ty: &TyKind,
        ttr: &Resolver<CompoundTypeData>,
        span: hir::Span,
    ) -> Self {
        let mut base_ty = base_ty;
        let mut result = InitializerCursor {
            base_ty: base_ty.clone(),
            stack: vec![],
            span,
        };
        for designator in designators {
            match &designator.kind {
                DesignatorKind::Subscript { value } => {
                    match base_ty {
                        TyKind::Array { kind, size: _ } => {
                            base_ty = kind;
                        }
                        _ => panic!("Invalid array designator for type {base_ty:?}"),
                    }
                    result.stack.push((*value as usize, base_ty.clone()));
                }
                DesignatorKind::Field { name } => {
                    let fields = base_ty.fields(ttr, span).unwrap();
                    let addr = &fields.by_name[name];
                    for &elem in addr {
                        let fields = base_ty.fields(ttr, span).unwrap();
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
        value: hir::ExprOrList,
        ctx: &mut HirCtx,
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
        *current = ctx.lower_to_initializer_tree(self.ty(), value, self.span);
    }

    /// Go down to the first non compound type to initialize.
    fn go_through_primitive(&mut self, ttr: &Resolver<CompoundTypeData>) {
        if self.stack.is_empty() {
            return;
        }
        match self.ty() {
            ty @ (TyKind::Struct(_) | TyKind::Union(_)) => {
                let fields = ty.fields(ttr, self.span).unwrap();
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
                let fields = ty.fields(ttr, self.span).unwrap();
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
    fn to_first(ttr: &Resolver<CompoundTypeData>, base_ty: &TyKind, span: hir::Span) -> Self {
        match base_ty {
            ty @ (TyKind::Struct(_) | TyKind::Union(_)) => {
                let fields = ty.fields(ttr, span).unwrap();
                Self {
                    span,
                    base_ty: base_ty.clone(),
                    stack: vec![(0, fields.by_index[0].kind.clone())],
                }
            }
            TyKind::Array { kind, size: _ } => Self {
                span,
                base_ty: base_ty.clone(),
                stack: vec![(0, (**kind).clone())],
            },
            _ => panic!("Invalid type {base_ty:?} for initializer list."),
        }
    }
}

impl<'hir> HirCtx<'hir> {
    /// Convert a hir expression to an initializer tree.
    pub(crate) fn lower_to_initializer_tree(
        &mut self,
        expected_ty: &TyKind,
        expr: hir::ExprOrList,
        span: hir::Span,
    ) -> InitializerTree {
        let list = match expr {
            ExprOrList::Expr(expr) => match expr.kind {
                ExprKind::Lit(ref lit) => {
                    if let LitKind::Str(string) = &lit.kind
                        && expected_ty.is_array()
                    {
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
                        return self.lower_to_initializer_tree(expected_ty, init_expr, span);
                    } else {
                        return InitializerTree::Leaf(expr);
                    }
                }
                _ => {
                    return InitializerTree::Leaf(expr);
                }
            },
            ExprOrList::List(list) => list,
        };
        let mut result = InitializerTree::Middle { children: vec![] };
        let mut cursor = InitializerCursor::to_first(&self.type_tag_resolver, expected_ty, span);
        for item in list {
            if let Some(designators) = &item.designators {
                cursor = InitializerCursor::from_designators(
                    designators,
                    expected_ty,
                    &self.type_tag_resolver,
                    span,
                );
            }
            if !matches!(item.value, ExprOrList::List(_)) {
                cursor.go_through_primitive(&self.type_tag_resolver);
            }
            cursor.insert_to_tree(&mut result, item.value, self);
            cursor.go_next(&self.type_tag_resolver);
        }
        result
    }
}

/// Construct an initializer list from a string literal.
pub(crate) fn initializer_list_from_string(
    string: &str,
    ty: hir::Ty,
    span: hir::Span,
) -> hir::ExprOrList {
    use hir::{Expr, ExprKind, InitializerItem, Lit, LitKind};
    ExprOrList::List(
        string
            .chars()
            .chain([0 as char])
            .map(|ch| InitializerItem {
                designators: None,
                value: ExprOrList::Expr(Expr {
                    kind: ExprKind::Lit(Lit {
                        kind: LitKind::Char(ch),
                        span,
                    }),
                    ty: ty.clone(),
                    span,
                }),
            })
            .collect(),
    )
}
