#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use log::trace;

use crate::hir::{constants, datatypes::*};

impl HirCtx<'_> {
    fn lower_to_prim_ty_kind(&mut self) -> anyhow::Result<PrimTyKind> {
        let node = self.cursor.node();
        trace!("Construct [PrimTyKind] from node: {}", node.kind());

        Ok(
            match std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])? {
                constants::INT => PrimTyKind::Int,
                constants::BOOL => PrimTyKind::Bool,
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
            constants::PRIMITIVE_TYPE | constants::TYPE_IDENTIFIER => {
                TyKind::PrimTy(self.lower_to_prim_ty_kind()?)
            }
            constants::TYPE_DESCRIPTOR => {
                self.cursor.goto_first_child();

                let ty_kind = self.lower_to_ty_kind()?;

                self.cursor.goto_parent();

                ty_kind
            }
            kind => bail!("Unsupported [TyKind] node: {kind}"),
        })
    }

    fn lower_to_ty_qual(&mut self) -> anyhow::Result<TyQual> {
        let node = self.cursor.node();
        trace!("Construct [TyQual] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::CONST => TyQual::Const,
            constants::VOLATILE => TyQual::Volatile,
            constants::ATOMIC => TyQual::Atomic,
            constants::LINEAR => TyQual::Linear,
            kind => bail!("Unsupported [TyQual] node: {kind}"),
        })
    }

    pub(crate) fn lower_to_ty(&mut self) -> anyhow::Result<Ty> {
        let node = self.cursor.node();
        trace!("Construct [Ty] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut quals = vec![];

        while self.cursor.node().kind() == constants::TYPE_QUALIFIER {
            self.cursor.goto_first_child();

            quals.push(self.lower_to_ty_qual()?);

            self.cursor.goto_parent();
            self.cursor.goto_next_sibling();
        }

        Ok(Ty {
            kind: self.lower_to_ty_kind()?,
            quals,
            span,
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

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(Lit {
            kind: self.lower_to_lit_kind()?,
            span,
        })
    }

    pub(crate) fn lower_to_ident(&mut self) -> anyhow::Result<Ident> {
        let node = self.cursor.node();
        trace!("Construct [Ident] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(Ident {
            name: std::str::from_utf8(
                &self.source_code[self.cursor.node().start_byte()..self.cursor.node().end_byte()],
            )?
            .to_string(),
            span,
        })
    }
}
