#![allow(clippy::missing_docs_in_private_items)]

use log::trace;

use crate::{constants, datatypes::*};

impl LoweringCtx<'_> {
    pub(crate) fn lower_param(&mut self) -> anyhow::Result<Param> {
        let node = self.cursor.node();
        trace!("Construct [Param] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ty = self.lower_ty()?;

        if self.cursor.goto_next_sibling() {
            let ident = self.lower_ident()?;

            let node = self.cursor.node();

            let idx = self.var_arena.alloc(DeclStmt {
                ty: ty.clone(),
                ident: ident.clone(),
                init: None,
                span: Span {
                    lo: node.start_byte(),
                    hi: node.end_byte(),
                },
            });
            self.var_map.insert(ident.name, idx);
        }

        self.cursor.goto_parent();

        Ok(Param { ty })
    }

    pub(crate) fn lower_fn_sig(&mut self) -> anyhow::Result<FnSig> {
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

        let idx = self.fn_arena.alloc(fn_sig.clone());
        self.fn_map.insert(ident.name, idx);

        Ok(fn_sig)
    }

    pub(crate) fn lower_fn(&mut self) -> anyhow::Result<Fn> {
        let node = self.cursor.node();
        trace!("Construct [Fn] from node: {}", node.kind());

        let previous_var_map = self.var_map.clone();

        let sig = self.lower_fn_sig()?;

        self.cursor.goto_last_child();

        let body = self.lower_expr()?;

        self.var_map = previous_var_map;

        self.cursor.goto_parent();

        Ok(Fn { sig, body })
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
