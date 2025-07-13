#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::bail;
use log::trace;

use crate::hir::{constants, datatypes::*, resolver::ResKind};

impl LoweringCtx<'_> {
    fn process_decl(&mut self, mut ty: Ty) -> anyhow::Result<(Ty, Ident)> {
        let node = self.cursor.node();
        trace!("Process [DeclStmt] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::ARRAY_DECLARATOR => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let array_len = self.lower_to_expr()?;

                let span = ty.span;

                ty = Ty {
                    kind: TyKind::Array(Box::new(ty), Box::new(array_len)),
                    span,
                };

                self.cursor.goto_previous_sibling();
                self.cursor.goto_previous_sibling();

                let result = self.process_decl(ty)?;

                self.cursor.goto_parent();

                result
            }
            constants::POINTER_DECLARATOR => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ident = self.lower_to_ident()?;

                self.cursor.goto_parent();

                let span = ty.span;

                (
                    Ty {
                        kind: TyKind::Ptr(Box::new(ty)),
                        span,
                    },
                    ident,
                )
            }
            _ => (ty, self.lower_to_ident()?),
        })
    }

    fn lower_to_decl_stmt(&mut self) -> anyhow::Result<Vec<DeclStmt>> {
        let node = self.cursor.node();
        trace!("Construct [DeclStmt] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ty = self.lower_to_ty()?;

        self.cursor.goto_next_sibling();

        let mut decl_stmts = vec![];

        loop {
            let ty = ty.clone();

            let (ty, ident, init) = match self.cursor.node().kind() {
                constants::INIT_DECLARATOR => {
                    self.cursor.goto_first_child();

                    let (ty, ident) = self.process_decl(ty)?;

                    self.cursor.goto_next_sibling();
                    self.cursor.goto_next_sibling();

                    let init = self.lower_to_expr()?;

                    self.cursor.goto_parent();

                    (ty, ident, Some(init))
                }
                _ => {
                    let (ty, ident) = self.process_decl(ty)?;

                    (ty, ident, None)
                }
            };

            let idx = self.resolver.insert(ident, ResKind::Local(ty.clone()))?;

            let decl_stmt = DeclStmt {
                res: idx,
                ty,
                init,
                span: Span {
                    lo: node.start_byte(),
                    hi: node.end_byte(),
                },
            };

            decl_stmts.push(decl_stmt);

            self.cursor.goto_next_sibling();
            if !self.cursor.goto_next_sibling() {
                break;
            }
        }

        self.cursor.goto_parent();

        Ok(decl_stmts)
    }

    fn lower_to_stmt_kind(&mut self) -> anyhow::Result<Vec<StmtKind>> {
        let node = self.cursor.node();
        trace!("Construct [StmtKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::DECLARATION => self
                .lower_to_decl_stmt()?
                .into_iter()
                .map(StmtKind::Decl)
                .collect(),
            constants::RETURN_STATEMENT
            | constants::EXPRESSION_STATEMENT
            | constants::BREAK_STATEMENT
            | constants::CONTINUE_STATEMENT => {
                vec![StmtKind::Semi(self.lower_to_expr()?)]
            }
            constants::IF_STATEMENT
            | constants::WHILE_STATEMENT
            | constants::DO_STATEMENT
            | constants::FOR_STATEMENT
            | constants::COMPOUND_STATEMENT => {
                vec![StmtKind::Expr(self.lower_to_expr()?)]
            }
            kind => bail!("Unsupported [StmtKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_to_stmt(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let node = self.cursor.node();
        trace!("Construct [Stmt] from node: {}", node.kind());

        Ok(self
            .lower_to_stmt_kind()?
            .into_iter()
            .map(|stmt_kind| Stmt {
                kind: stmt_kind,
                span: Span {
                    lo: node.start_byte(),
                    hi: node.end_byte(),
                },
            })
            .collect())
    }

    pub(crate) fn lower_to_block(&mut self) -> anyhow::Result<Block> {
        let node = self.cursor.node();
        trace!("Construct [Block] from node: {}", node.kind());

        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let pre_resolver = self.resolver.clone();

        let mut stmts = vec![];

        while self.cursor.node().kind() != "}" {
            stmts.append(&mut self.lower_to_stmt()?);

            self.cursor.goto_next_sibling();
        }

        self.cursor.goto_parent();

        let resolver = mem::replace(&mut self.resolver, pre_resolver);

        Ok(Block {
            stmts,
            resolver,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}
