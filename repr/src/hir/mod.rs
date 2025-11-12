//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!

use log::error;
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
/// Contains methods needed to lower to item.
mod item;
/// Contains methods needed to lower to statement.
mod stmt;
/// Contains methods needed to lower to type.
mod ty;

pub use decl::*;
pub use expr::*;
pub use item::*;
pub use stmt::*;
pub use ty::*;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

pub struct HirCtx<'hir> {
    pub symbol_resolver: resolver::Resolver<resolver::SymbolKind>,
    pub type_tag_resolver: resolver::Resolver<resolver::CompoundTypeData>,
    pub label_resolver: resolver::Resolver<()>,

    pub items: Vec<Item>,

    pub switch_cond: Option<Expr>,
    pub start_label: Option<resolver::Label>,
    pub end_label: Option<resolver::Label>,

    pub root: Node<'hir>,
    pub source_code: &'hir [u8],
}

impl<'hir> HirCtx<'hir> {
    pub fn new(ast_repr: &'hir AstRepr) -> Self {
        Self {
            symbol_resolver: resolver::Resolver::new(),
            type_tag_resolver: resolver::Resolver::new(),
            label_resolver: resolver::Resolver::new(),

            items: vec![],

            switch_cond: None,
            start_label: None,
            end_label: None,

            root: ast_repr.tree.root_node(),
            source_code: &ast_repr.source_info.code,
        }
    }

    pub fn lower_to_hir(
        mut self,
    ) -> (
        Vec<Item>,
        resolver::Resolver<resolver::SymbolKind>,
        resolver::Resolver<resolver::CompoundTypeData>,
    ) {
        let mut cursor = self.root.walk();

        for child in self.root.children(&mut cursor) {
            match self.lower_to_item(child) {
                Ok(item) => {
                    self.items.push(item);
                }
                Err(error) => {
                    error!("Failed to construct 'HIR' - {error:?}");
                }
            }
        }

        (self.items, self.symbol_resolver, self.type_tag_resolver)
    }
}
