//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!

use tree_sitter::Node;

use ast_utils::AstRepr;

/// Contains constant identifiers used to generate the HIR.
pub(crate) mod constants;
/// Contains symbol resolver's implementation.
pub mod resolver;

/// Contains methods needed to lower to declaration
mod decl;
/// Contains methods needed to lower to expression.
mod expr;
/// Contains methods needed to lower to initializer list tree.
mod initializer_tree;
/// Contains methods needed to lower to item.
mod item;
/// Contains methods needed to lower to statement.
mod stmt;
/// Contains methods needed to lower to type.
mod ty;

pub use decl::*;
pub use expr::*;
pub use initializer_tree::*;
pub use item::*;
pub use stmt::*;
pub use ty::*;

use crate::hir::resolver::{CompoundTypeData, Label, Resolver, SymbolKind};
pub use azhdaha_errors::Span;

#[derive(Default)]
pub struct SwitchData {
    /// `case x:` with evaluated `x`.
    cases: Vec<(i32, Label)>,
    /// `default:`
    default_case: Option<Label>,
}

pub struct HirCtx<'hir> {
    pub symbol_resolver: Resolver<SymbolKind>,
    pub type_tag_resolver: Resolver<CompoundTypeData>,
    pub label_resolver: Resolver<()>,

    pub items: Vec<Item>,

    pub switch_data: Option<SwitchData>,
    pub start_label: Option<resolver::Label>,
    pub end_label: Option<resolver::Label>,

    pub return_ty: Option<Ty>,

    pub root: Node<'hir>,
    pub source_code: &'hir [u8],
}

fn default_symbol_resolver() -> Resolver<SymbolKind> {
    let mut result = Resolver::new();
    let list = [
        ("__builtin_bswap16", 2),
        ("__builtin_bswap32", 4),
        ("__builtin_bswap64", 8),
    ];

    for (name, size) in list {
        result.insert_symbol(
            name.to_owned(),
            SymbolKind::Var(VarDecl {
                storage: None,
                ident: Ident {
                    name: name.to_owned(),
                    span: Span::DUMMY,
                },
                ty: Ty {
                    kind: TyKind::Func {
                        sig: Box::new(FuncSig {
                            ret_ty: Ty {
                                kind: TyKind::PrimTy(PrimTyKind::Int(size)),
                                is_linear: false,
                                quals: vec![],
                                span: Span::DUMMY,
                            },
                            params: vec![ParamDecl {
                                storage: None,
                                ident: None,
                                ty: Ty {
                                    kind: TyKind::PrimTy(PrimTyKind::Int(size)),
                                    is_linear: false,
                                    quals: vec![],
                                    span: Span::DUMMY,
                                },
                                span: Span::DUMMY,
                            }],
                            variadic_param: false,
                            span: Span::DUMMY,
                        }),
                    },
                    is_linear: false,
                    quals: vec![],
                    span: Span::DUMMY,
                },
                init: None,
                span: Span::DUMMY,
            }),
        );
    }
    result
}

impl<'hir> HirCtx<'hir> {
    pub fn new(ast_repr: &'hir AstRepr) -> Self {
        Self {
            symbol_resolver: default_symbol_resolver(),
            type_tag_resolver: Resolver::new(),
            label_resolver: Resolver::new(),

            items: vec![],

            switch_data: None,
            start_label: None,
            end_label: None,

            return_ty: None,

            root: ast_repr.tree.root_node(),
            source_code: &ast_repr.source_info.code,
        }
    }

    /// # Panics
    /// This function panics if the source is not utf8.
    pub fn lower_to_hir(
        mut self,
    ) -> (
        Vec<Item>,
        Resolver<SymbolKind>,
        Resolver<CompoundTypeData>,
        bool,
    ) {
        let mut cursor = self.root.walk();
        let mut had_errors = false;

        for child in self.root.children(&mut cursor) {
            match self.lower_to_item(child) {
                Ok(item) => {
                    self.items.push(item);
                }
                Err(error) => {
                    had_errors = true;
                    error.report(std::str::from_utf8(self.source_code).unwrap());
                }
            }
        }

        (
            self.items,
            self.symbol_resolver,
            self.type_tag_resolver,
            had_errors,
        )
    }
}
