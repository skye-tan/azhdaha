//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!
//! This implementation has been modeled after rustc's HIR.
//!

use std::collections::HashMap;

use la_arena::{Arena, Idx};

use ast_utils::AstRepr;

/// Contains the methods needed to lower [`Block`], [`Stmt`], [`StmtKind`], and [`DeclStmt`].
mod block;
/// Contains the methods needed to lower [`Expr`], [`ExprKind`], [`Sizeof`], [`SizeofKind`], [`UnOp`], [`BinOp`], and [`BinOpKind`].
mod expr;
/// Contains the methods needed to lower [`Item`], [`ItemKind`], [`Fn`], [`FnSig`], and [`Param`].
mod item;
/// Contains the methods needed to lower [`Path`], [`PrimTyKind`], [`TyKind`], [`Ty`], [`Ident`], [`LitKind`], and [`Lit`].
mod path;

/// Contains constant values used to generate the HIR.
mod constants;
/// Contains datatypes used to represent the HIR.
mod datatypes;

pub use datatypes::*;

impl Resolver {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, item: Ty) {
        let idx = self.arena.alloc(item);

        self.map.insert(key, idx);
    }

    pub fn lookup_idx(&mut self, key: &str) -> Option<Idx<Ty>> {
        self.map.get(key).cloned()
    }

    pub fn get_item(&mut self, idx: Idx<Ty>) -> Ty {
        self.arena[idx].clone()
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl<'hir> LoweringCtx<'hir> {
    pub fn lower_ast(ast_repr: &'hir AstRepr) -> Self {
        let mut lowering_ctx = Self {
            items: vec![],
            resolver: Resolver::new(),
            cursor: ast_repr.tree.walk(),
            source_code: &ast_repr.source_code,
        };

        lowering_ctx.cursor.goto_first_child();

        loop {
            match lowering_ctx.lower_item() {
                Ok(item) => {
                    if let Some(item) = item {
                        lowering_ctx.items.push(item);
                    }
                }
                Err(error) => {
                    log::warn!("Failed to construct item - {:?}", error);
                }
            }

            if !lowering_ctx.cursor.goto_next_sibling() {
                break;
            }
        }

        lowering_ctx
    }
}
