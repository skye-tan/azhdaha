#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::bail;
use log::trace;

use crate::hir::*;

use super::{
    constants,
    resolver::{Resolver, Symbol, SymbolKind},
};

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Func(Box<FuncDef>),
    Decl(Symbol),
    TyDef(Symbol),
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub symbol_resolver: Resolver<SymbolKind>,
    pub label_resolver: Resolver<()>,

    pub symbol: Symbol,
    pub body: Stmt,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub symbol_resolver: Resolver<SymbolKind>,

    pub stmts: Vec<Stmt>,
    pub span: Span,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_item(&mut self, node: Node) -> anyhow::Result<Item> {
        trace!("[HIR/Item] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let kind = self.lower_to_item_kind(node)?;

        Ok(Item { kind, span })
    }

    pub(crate) fn lower_to_item_kind(&mut self, node: Node) -> anyhow::Result<ItemKind> {
        trace!("[HIR/ItemKind] Lowering '{}'", node.kind());

        Ok(match node.kind() {
            constants::FUNCTION_DEFINITION => {
                ItemKind::Func(Box::new(self.lower_to_func_def(node)?))
            }
            constants::DECLARATION => {
                let local_decl = self.lower_to_local_decl(node)?;

                let symbol = self
                    .symbol_resolver
                    .insert_symbol(local_decl.ident.name.clone(), SymbolKind::Local(local_decl));

                ItemKind::Decl(symbol)
            }
            constants::TYPE_DEFINITION => {
                let local_decl: LocalDecl = self.lower_to_local_decl(node)?;

                let symbol = self
                    .symbol_resolver
                    .insert_symbol(local_decl.ident.name, SymbolKind::TyDef(local_decl.ty));

                ItemKind::TyDef(symbol)
            }
            kind => {
                bail!("Cannot lower '{kind}' to 'ItemKind'.");
            }
        })
    }

    pub(crate) fn lower_to_func_def(&mut self, node: Node) -> anyhow::Result<FuncDef> {
        trace!("[HIR/FuncDef] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut saved_symbol_resolver = self.symbol_resolver.clone();

        let func_decl = self.lower_to_func_decl(node)?;

        _ = self.symbol_resolver.insert_symbol(
            func_decl.ident.name.clone(),
            SymbolKind::Func(func_decl.clone()),
        );

        for param in &func_decl.sig.params {
            if let Some(ident) = &param.ident {
                _ = self
                    .symbol_resolver
                    .insert_symbol(ident.name.clone(), SymbolKind::Param(param.clone()));
            }
        }

        let symbol = saved_symbol_resolver
            .insert_symbol(func_decl.ident.name.clone(), SymbolKind::Func(func_decl));

        let body = self.lower_to_stmt(node.child(node.child_count() - 1).unwrap())?;

        let symbol_resolver = mem::replace(&mut self.symbol_resolver, saved_symbol_resolver);
        let label_resolver = mem::take(&mut self.label_resolver);

        Ok(FuncDef {
            symbol_resolver,
            label_resolver,
            symbol,
            body,
            span,
        })
    }

    pub(crate) fn lower_to_block(&mut self, node: Node) -> anyhow::Result<Block> {
        trace!("[HIR/Block] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let saved_symbol_resolver = self.symbol_resolver.clone();

        let mut stmts = vec![];

        let mut cursor = node.walk();
        cursor.goto_first_child();
        cursor.goto_next_sibling();

        while cursor.node().kind() != "}" {
            let stmt = self.lower_to_stmt(cursor.node())?;

            stmts.push(stmt);

            cursor.goto_next_sibling();
        }

        let symbol_resolver = mem::replace(&mut self.symbol_resolver, saved_symbol_resolver);

        Ok(Block {
            symbol_resolver,
            stmts,
            span,
        })
    }
}
