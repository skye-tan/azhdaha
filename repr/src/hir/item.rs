#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::{Context, bail};
use log::trace;

use crate::hir::{constants, datatypes::*, resolver::SymbolKind};

impl HirCtx<'_> {
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
        let mut variadic_param = false;

        while self.cursor.node().kind() != ")" {
            if self.cursor.node().kind() == "variadic_parameter" {
                variadic_param = true;
            } else {
                params.push(self.lower_to_param()?);
            }

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
            variadic_param,
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

        let symbol_resolver = mem::replace(&mut self.symbol_resolver, saved_symbol_resolver);
        let label_resolver = mem::take(&mut self.label_resolver);

        Ok(Func {
            symbol_resolver,
            label_resolver,
            sig: symbol,
            body,
        })
    }

    pub(crate) fn lower_to_item_kind(&mut self) -> anyhow::Result<ItemKind> {
        let node = self.cursor.node();
        trace!("Construct [ItemKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::FUNCTION_DEFINITION => ItemKind::Func(Box::new(self.lower_to_func()?)),
            constants::DECLARATION => {
                let child_node = node.child(1).context("Unknown declaration.")?;

                match child_node.kind() {
                    constants::FUNCTION_DECLARATOR => {
                        let func_sig = self.lower_to_func_sig()?;

                        let symbol = self
                            .symbol_resolver
                            .insert_symbol(func_sig.ident.name.clone(), SymbolKind::Func(func_sig));

                        ItemKind::ProtoType(symbol)
                    }
                    _ => ItemKind::GlobalVar(self.lower_to_decl_stmt()?),
                }
            }
            kind => {
                bail!("Unsupported [ItemKind] node: {kind}");
            }
        })
    }

    pub(crate) fn lower_to_item(&mut self) -> anyhow::Result<Item> {
        let node = self.cursor.node();
        trace!("Construct [Item] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let item_kind = self.lower_to_item_kind()?;

        Ok(Item {
            kind: item_kind,
            span,
        })
    }
}
