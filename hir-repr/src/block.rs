#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use log::trace;

use crate::{constant, datatype::*};

impl LoweringCtx<'_> {
    fn process_decl(&mut self, mut ty: Ty) -> anyhow::Result<(Ty, Ident)> {
        let node = self.cursor.node();
        trace!("Process [DeclStmt] from node: {}", node.kind());

        Ok(match node.kind() {
            constant::ARRAY_DECLARATOR => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let array_len = self.lower_expr()?;

                let span = ty.span.clone();

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
            constant::POINTER_DECLARATOR => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ident = self.lower_ident()?;

                self.cursor.goto_parent();

                let span = ty.span.clone();

                (
                    Ty {
                        kind: TyKind::Ptr(Box::new(ty)),
                        span,
                    },
                    ident,
                )
            }
            _ => (ty, self.lower_ident()?),
        })
    }

    fn lower_decl_stmt(&mut self) -> anyhow::Result<Vec<DeclStmt>> {
        let node = self.cursor.node();
        trace!("Construct [DeclStmt] from node: {}", node.kind());

        self.cursor.goto_first_child();

        let ty = self.lower_ty()?;

        self.cursor.goto_next_sibling();

        let mut decl_stmts = vec![];

        loop {
            let ty = ty.clone();

            let (ty, ident, init) = match self.cursor.node().kind() {
                constant::INIT_DECLARATOR => {
                    self.cursor.goto_first_child();

                    let (ty, ident) = self.process_decl(ty)?;

                    self.cursor.goto_next_sibling();
                    self.cursor.goto_next_sibling();

                    let init = self.lower_expr()?;

                    self.cursor.goto_parent();

                    (ty, ident, Some(init))
                }
                _ => {
                    let (ty, ident) = self.process_decl(ty)?;

                    (ty, ident, None)
                }
            };

            decl_stmts.push(DeclStmt {
                ty,
                ident,
                init,
                span: Span {
                    lo: node.start_byte(),
                    hi: node.end_byte(),
                },
            });

            self.cursor.goto_next_sibling();
            if !self.cursor.goto_next_sibling() {
                break;
            }
        }

        self.cursor.goto_parent();

        Ok(decl_stmts)
    }

    fn lower_stmt_kind(&mut self) -> anyhow::Result<Vec<StmtKind>> {
        let node = self.cursor.node();
        trace!("Construct [StmtKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constant::DECLARATION => self
                .lower_decl_stmt()?
                .into_iter()
                .map(StmtKind::Decl)
                .collect(),
            constant::RETURN_STATEMENT
            | constant::EXPRESSION_STATEMENT
            | constant::BREAK_STATEMENT
            | constant::CONTINUE_STATEMENT => {
                vec![StmtKind::Semi(self.lower_expr()?)]
            }
            constant::IF_STATEMENT
            | constant::WHILE_STATEMENT
            | constant::DO_STATEMENT
            | constant::FOR_STATEMENT => {
                vec![StmtKind::Expr(self.lower_expr()?)]
            }
            kind => bail!("Unsupported [StmtKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_stmt(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let node = self.cursor.node();
        trace!("Construct [Stmt] from node: {}", node.kind());

        Ok(self
            .lower_stmt_kind()?
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

    pub(crate) fn lower_block(&mut self) -> anyhow::Result<Block> {
        let node = self.cursor.node();
        trace!("Construct [Block] from node: {}", node.kind());

        let res_ctx = ResCtx::new();

        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let mut stmts = vec![];

        while self.cursor.node().kind() != "}" {
            stmts.append(&mut self.lower_stmt()?);

            self.cursor.goto_next_sibling();
        }

        self.cursor.goto_parent();

        Ok(Block {
            stmts,
            res_ctx,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}
