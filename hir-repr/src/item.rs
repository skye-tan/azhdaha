#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use log::trace;

use crate::{constants, datatypes::*};

impl LoweringCtx<'_> {
    pub(crate) fn lower_param(&mut self) -> anyhow::Result<Param> {
        let node = self.cursor.node();
        trace!("Construct [Param] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ty = self.lower_ty()?;

        let ident = if self.cursor.goto_next_sibling() {
            let ident = self.lower_ident()?;

            self.resolver
                .insert(ident.name.clone(), ResolverData::Local(ty.clone()))?;

            Some(ident)
        } else {
            None
        };

        self.cursor.goto_parent();

        Ok(Param { ty, ident })
    }

    pub(crate) fn lower_fn_sig(&mut self) -> anyhow::Result<(Resolver, FnSig)> {
        let node = self.cursor.node();
        trace!("Construct [FnSig] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ty = self.lower_ty()?;

        self.cursor.goto_next_sibling();
        self.cursor.goto_first_child();

        let ident = self.lower_ident()?;

        self.cursor.goto_next_sibling();
        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let mut pre_resolver = self.resolver.clone();

        let mut params = vec![];

        while self.cursor.node().kind() != ")" {
            params.push(self.lower_param()?);

            self.cursor.goto_next_sibling();
            self.cursor.goto_next_sibling();
        }

        self.cursor.goto_parent();
        self.cursor.goto_parent();
        self.cursor.goto_parent();

        let fn_sig = FnSig { ty, params };

        pre_resolver.insert(ident.name, ResolverData::Fn(fn_sig.clone()))?;

        Ok((pre_resolver, fn_sig))
    }

    pub(crate) fn lower_fn(&mut self) -> anyhow::Result<Fn> {
        let node = self.cursor.node();
        trace!("Construct [Fn] from node: {}", node.kind());

        let (pre_resolver, sig) = self.lower_fn_sig()?;

        self.cursor.goto_last_child();

        let body = self.lower_expr()?;

        self.cursor.goto_parent();

        let resolver = mem::replace(&mut self.resolver, pre_resolver);

        Ok(Fn {
            sig,
            body,
            resolver,
        })
    }

    pub(crate) fn lower_item_kind(&mut self) -> anyhow::Result<Option<ItemKind>> {
        let node = self.cursor.node();
        trace!("Construct [ItemKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::FUNCTION_DEFINITION => Some(ItemKind::Fn(self.lower_fn()?)),
            kind => {
                trace!("Unsupported [ItemKind] node: {kind}");
                None
            }
        })
    }

    pub(crate) fn lower_item(&mut self) -> anyhow::Result<Option<Item>> {
        let node = self.cursor.node();
        trace!("Construct [Item] from node: {}", node.kind());

        Ok(self.lower_item_kind()?.map(|item_kind| Item {
            kind: item_kind,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        }))
    }
}
