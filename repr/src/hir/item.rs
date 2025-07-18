#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::Context;
use log::trace;

use crate::hir::{constants, datatypes::*, resolver::SymbolKind};

impl LoweringCtx<'_> {
    pub(crate) fn lower_to_param(&mut self) -> anyhow::Result<Param> {
        let node = self.cursor.node();
        trace!("Construct [Param] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        self.cursor.goto_first_child();

        let ty = self.lower_to_ty()?;

        let ident = if self.cursor.goto_next_sibling() {
            Some(self.lower_to_ident()?)
        } else {
            None
        };

        self.cursor.goto_parent();

        Ok(Param { ty, ident, span })
    }

    pub(crate) fn lower_to_func_sig(&mut self) -> anyhow::Result<FuncSig> {
        let node = self.cursor.node();
        trace!("Construct [FuncSig] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ret_ty = self.lower_to_ty()?;

        self.cursor.goto_next_sibling();
        self.cursor.goto_first_child();

        let ident = self.lower_to_ident()?;

        self.cursor.goto_next_sibling();
        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let mut params = vec![];

        while self.cursor.node().kind() != ")" {
            params.push(self.lower_to_param()?);

            self.cursor.goto_next_sibling();
            self.cursor.goto_next_sibling();
        }

        self.cursor.goto_parent();
        self.cursor.goto_parent();
        self.cursor.goto_parent();

        Ok(FuncSig {
            ret_ty,
            ident,
            params,
        })
    }

    pub(crate) fn lower_to_func(&mut self) -> anyhow::Result<Func> {
        let node = self.cursor.node();
        trace!("Construct [Func] from node: {}", node.kind());

        let func_sig = self.lower_to_func_sig()?;

        let symbol = self.symbol_resolver.insert_symbol(
            func_sig.ident.name.clone(),
            SymbolKind::Func(func_sig.clone()),
        );

        let saved_symbol_resolver = self.symbol_resolver.clone();

        for param in func_sig.params.into_iter() {
            if let Some(ident) = param.ident {
                _ = self.symbol_resolver.insert_symbol(
                    ident.name.clone(),
                    SymbolKind::Local(Decl {
                        ty: param.ty,
                        ident,
                        init: None,
                        span: param.span,
                    }),
                );
            }
        }

        self.cursor.goto_last_child();

        let body = self.lower_to_stmt()?;

        self.cursor.goto_parent();

        self.symbol_resolver = saved_symbol_resolver;
        let label_resolver = mem::take(&mut self.label_resolver);

        Ok(Func {
            label_resolver,
            sig: symbol,
            body,
        })
    }

    pub(crate) fn lower_to_item_kind(&mut self) -> anyhow::Result<Option<ItemKind>> {
        let node = self.cursor.node();
        trace!("Construct [ItemKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::FUNCTION_DEFINITION => Some(ItemKind::Func(self.lower_to_func()?)),
            constants::DECLARATION => {
                let child_node = node.child(1).context("Unknown declaration.")?;

                match child_node.kind() {
                    constants::FUNCTION_DECLARATOR => {
                        let func_sig = self.lower_to_func_sig()?;

                        let symbol = self
                            .symbol_resolver
                            .insert_symbol(func_sig.ident.name.clone(), SymbolKind::Func(func_sig));

                        Some(ItemKind::ProtoType(symbol))
                    }
                    _ => Some(ItemKind::GlobalVar(self.lower_to_decl_stmt()?)),
                }
            }
            kind => {
                trace!("Unsupported [ItemKind] node: {kind}");
                None
            }
        })
    }

    pub(crate) fn lower_to_item(&mut self) -> anyhow::Result<Option<Item>> {
        let node = self.cursor.node();
        trace!("Construct [Item] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(self.lower_to_item_kind()?.map(|item_kind| Item {
            kind: item_kind,
            span,
        }))
    }
}
