//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!

use tree_sitter::Node;

use ast_utils::AstRepr;

/// Contains constant identifiers used to generate the HIR.
pub(crate) mod constants;
/// Contains symbol resolver's implementation.
pub(crate) mod resolver;

/// Contains the methods needed to lower to declaration
mod decl;
/// Contains the methods needed to lower to expression.
mod expr;
/// Contains the methods needed to lower to item.
mod item;
/// Contains the methods needed to lower to statement.
mod stmt;
/// Contains the methods needed to lower to type.
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
    pub label_resolver: resolver::Resolver<()>,

    pub items: Vec<Item>,

    pub root: Node<'hir>,
    pub source_code: &'hir [u8],
}

impl<'hir> HirCtx<'hir> {
    pub fn new(ast_repr: &'hir AstRepr) -> Self {
        Self {
            symbol_resolver: resolver::Resolver::new(),
            label_resolver: resolver::Resolver::new(),
            items: vec![],
            root: ast_repr.tree.root_node(),
            source_code: &ast_repr.source_info.code,
        }
    }

    pub fn lower_to_hir(mut self) -> Vec<Item> {
        let mut cursor = self.root.walk();

        for child in self.root.children(&mut cursor) {
            match self.lower_to_item(child) {
                Ok(item) => {
                    self.items.push(item);
                }
                Err(error) => {
                    log::warn!("Failed to construct 'HIR' - {error:?}");
                }
            }
        }

        self.items
    }
}
