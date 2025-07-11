#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use log::trace;

use crate::hir::{constants, datatypes::*};

impl LoweringCtx<'_> {
    fn lower_to_prim_ty_kind(&mut self) -> anyhow::Result<PrimTyKind> {
        let node = self.cursor.node();
        trace!("Construct [PrimTyKind] from node: {}", node.kind());

        Ok(
            match std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])? {
                constants::INT => PrimTyKind::Int,
                constants::FLOAT => PrimTyKind::Float,
                constants::DOUBLE => PrimTyKind::Double,
                constants::CHAR => PrimTyKind::Char,
                constants::VOID => PrimTyKind::Void,
                kind => bail!("Unsupported [PrimTyKind] node: {kind}"),
            },
        )
    }

    fn lower_to_ty_kind(&mut self) -> anyhow::Result<TyKind> {
        let node = self.cursor.node();
        trace!("Construct [TyKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::PRIMITIVE_TYPE => TyKind::PrimTy(self.lower_to_prim_ty_kind()?),
            constants::TYPE_DESCRIPTOR => {
                self.cursor.goto_first_child();

                let ty_kind = self.lower_to_ty_kind()?;

                self.cursor.goto_parent();

                ty_kind
            }
            kind => bail!("Unsupported [TyKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_to_ty(&mut self) -> anyhow::Result<Ty> {
        let node = self.cursor.node();
        trace!("Construct [Ty] from node: {}", node.kind());

        Ok(Ty {
            kind: self.lower_to_ty_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }

    pub(crate) fn lower_to_ident(&mut self) -> anyhow::Result<Ident> {
        let node = self.cursor.node();
        trace!("Construct [Ident] from node: {}", node.kind());

        Ok(Ident {
            name: std::str::from_utf8(
                &self.source_code[self.cursor.node().start_byte()..self.cursor.node().end_byte()],
            )?
            .to_string(),
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }

    fn lower_to_lit_kind(&mut self) -> anyhow::Result<LitKind> {
        let node = self.cursor.node();
        trace!("Construct [LitKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::STRING_LITERAL => LitKind::Str(
                std::str::from_utf8(&self.source_code[node.start_byte() + 1..node.end_byte() - 1])?
                    .to_owned(),
            ),
            constants::CHAR_LITERAL => {
                LitKind::Char(self.source_code[node.start_byte() + 1] as char)
            }
            constants::NUMBER_LITERAL => {
                let literal =
                    std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])?;

                if let Ok(value) = literal.parse() {
                    LitKind::Int(value)
                } else {
                    LitKind::Float(literal.parse()?)
                }
            }
            kind => bail!("Unsupported [LitKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_to_lit(&mut self) -> anyhow::Result<Lit> {
        let node = self.cursor.node();
        trace!("Construct [Lit] from node: {}", node.kind());

        Ok(Lit {
            kind: self.lower_to_lit_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }

    pub(crate) fn lower_to_path(&mut self) -> anyhow::Result<Path> {
        let node = self.cursor.node();
        trace!("Construct [Path] from node: {}", node.kind());

        let ident = self.lower_to_ident()?;

        let Some(res) = self.resolver.lookup_idx(&ident.name) else {
            bail!("Unknown identifier: {}", &ident.name);
        };

        Ok(Path {
            res,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}
