#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use log::trace;

use crate::hir::{
    constants,
    datatypes::*,
    resolver::{ResData, ResKind, Resolver},
};

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

        let res = if self.cursor.goto_next_sibling() {
            let ident = self.lower_to_ident()?;

            let ident_name = ident.name.clone();
            let res_data = ResData {
                ident,
                kind: ResKind::Var(ty.clone()),
            };
            let res = self.resolver.insert(ident_name, res_data)?;

            Some(res)
        } else {
            None
        };

        self.cursor.goto_parent();

        Ok(Param { res, ty, span })
    }

    pub(crate) fn lower_to_fn_sig(&mut self) -> anyhow::Result<(Resolver<ResData>, FnSig)> {
        let node = self.cursor.node();
        trace!("Construct [FnSig] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ty = self.lower_to_ty()?;

        self.cursor.goto_next_sibling();
        self.cursor.goto_first_child();

        let ident = self.lower_to_ident()?;

        self.cursor.goto_next_sibling();
        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let mut pre_resolver = self.resolver.clone();

        let mut params = vec![];

        while self.cursor.node().kind() != ")" {
            params.push(self.lower_to_param()?);

            self.cursor.goto_next_sibling();
            self.cursor.goto_next_sibling();
        }

        self.cursor.goto_parent();
        self.cursor.goto_parent();
        self.cursor.goto_parent();

        let ident_name = ident.name.clone();
        let res_data = ResData {
            ident,
            kind: ResKind::Fn(ty.clone(), params.clone()),
        };
        let res = pre_resolver.insert(ident_name, res_data)?;

        let fn_sig = FnSig { res, ty, params };

        Ok((pre_resolver, fn_sig))
    }

    pub(crate) fn lower_to_fn(&mut self) -> anyhow::Result<Fn> {
        let node = self.cursor.node();
        trace!("Construct [Fn] from node: {}", node.kind());

        let (pre_resolver, sig) = self.lower_to_fn_sig()?;

        self.cursor.goto_last_child();

        let body = self.lower_to_stmt()?;

        self.cursor.goto_parent();

        let resolver = mem::replace(&mut self.resolver, pre_resolver);
        let label_resolver = mem::take(&mut self.label_resolver);

        Ok(Fn {
            sig,
            body,
            resolver,
            label_resolver,
        })
    }

    pub(crate) fn lower_to_item_kind(&mut self) -> anyhow::Result<Option<ItemKind>> {
        let node = self.cursor.node();
        trace!("Construct [ItemKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::FUNCTION_DEFINITION => Some(ItemKind::Fn(Box::new(self.lower_to_fn()?))),
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
