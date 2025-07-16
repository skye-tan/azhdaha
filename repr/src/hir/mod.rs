//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!
//! This implementation has been modeled after rustc's HIR.
//!

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
/// Contains symbol-resolver's implementation.
pub(crate) mod resolver;

pub use datatypes::*;

impl<'hir> LoweringCtx<'hir> {
    pub fn lower_ast(ast_repr: &'hir AstRepr) -> Self {
        let mut lowering_ctx = Self {
            items: vec![],
            resolver: resolver::Resolver::new(),
            label_resolver: resolver::Resolver::new(),
            cursor: ast_repr.tree.walk(),
            source_code: &ast_repr.source_code,
        };

        lowering_ctx.cursor.goto_first_child();

        loop {
            match lowering_ctx.lower_to_item() {
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
