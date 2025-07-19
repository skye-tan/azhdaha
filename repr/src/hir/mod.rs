//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!
//! This implementation has been modeled after rustc's HIR.
//!

use ast_utils::AstRepr;

/// Contains the methods needed to lower ast to HIR's [`Block`], [`Stmt`], [`StmtKind`], and [`DeclStmt`].
mod block;
/// Contains the methods needed to lower ast to HIR's [`Expr`], [`ExprKind`], [`Sizeof`], [`SizeofKind`], [`UnOp`], and [`BinOp`].
mod expr;
/// Contains the methods needed to lower ast to HIR's [`Item`], [`ItemKind`], [`Fn`], [`FnSig`], and [`Param`].
mod item;
/// Contains the methods needed to lower ast to HIR's [`PrimTyKind`], [`TyKind`], [`Ty`], [`Ident`], [`LitKind`], and [`Lit`].
mod local;

/// Contains constant values used to generate the HIR.
mod constants;
/// Contains datatypes used to represent the HIR.
mod datatypes;
/// Contains symbol resolver's implementation.
pub(crate) mod resolver;

pub use datatypes::*;

impl<'hir> HirCtx<'hir> {
    pub fn new(ast_repr: &'hir AstRepr) -> Self {
        Self {
            symbol_resolver: resolver::Resolver::new(),
            label_resolver: resolver::Resolver::new(),
            items: vec![],
            cursor: ast_repr.tree.walk(),
            source_code: &ast_repr.source_code,
        }
    }

    pub fn lower_to_hir(mut self) -> Vec<Item> {
        let mut root = self.cursor;

        for child in root.node().children(&mut root) {
            self.cursor = child.walk();

            match self.lower_to_item() {
                Ok(item) => {
                    self.items.push(item);
                }
                Err(error) => {
                    log::warn!("Failed to construct item - {error:?}");
                }
            }
        }

        self.items
    }
}
