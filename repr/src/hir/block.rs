#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::bail;
use log::trace;

use crate::hir::{
    constants,
    datatypes::*,
    resolver::{ResData, ResKind},
};

impl LoweringCtx<'_> {
    fn lower_to_decl(&mut self, ty: Ty) -> anyhow::Result<Decl> {
        let node = self.cursor.node();
        trace!("Process declaration from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let (ty, ident) = match node.kind() {
            constants::ARRAY_DECLARATOR => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let array_len = self.lower_to_expr()?;

                let ty_span = ty.span;
                let ty = Ty {
                    kind: TyKind::Array(Box::new(ty), Box::new(array_len)),
                    span: ty_span,
                };

                self.cursor.goto_previous_sibling();
                self.cursor.goto_previous_sibling();

                let result = self.lower_to_decl(ty)?;

                self.cursor.goto_parent();

                return Ok(result);
            }
            constants::POINTER_DECLARATOR => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ident = self.lower_to_ident()?;

                self.cursor.goto_parent();

                let ty_span = ty.span;
                let ty = Ty {
                    kind: TyKind::Ptr(Box::new(ty)),
                    span: ty_span,
                };

                (ty, ident)
            }
            _ => (ty, self.lower_to_ident()?),
        };

        let ident_name = ident.name.clone();
        let res_data = ResData {
            ident,
            kind: ResKind::Var(ty.clone()),
        };
        let res = self.resolver.insert(ident_name, res_data)?;

        Ok(Decl {
            ty,
            res,
            init: None,
            span,
        })
    }

    fn lower_to_decl_stmt(&mut self) -> anyhow::Result<DeclStmt> {
        let node = self.cursor.node();
        trace!("Construct [DeclStmt] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        self.cursor.goto_first_child();

        let ty = self.lower_to_ty()?;

        self.cursor.goto_next_sibling();

        let mut decls = vec![];

        loop {
            let ty = ty.clone();

            let decl = match self.cursor.node().kind() {
                constants::INIT_DECLARATOR => {
                    self.cursor.goto_first_child();

                    let mut decl = self.lower_to_decl(ty)?;

                    self.cursor.goto_next_sibling();
                    self.cursor.goto_next_sibling();

                    decl.init = Some(self.lower_to_expr()?);

                    self.cursor.goto_parent();

                    decl
                }
                _ => self.lower_to_decl(ty)?,
            };

            self.cursor.goto_next_sibling();

            decls.push(decl);

            if !self.cursor.goto_next_sibling() {
                break;
            }
        }

        self.cursor.goto_parent();

        Ok(DeclStmt { decls, span })
    }

    fn lower_to_block(&mut self) -> anyhow::Result<Block> {
        let node = self.cursor.node();
        trace!("Construct [Block] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let pre_resolver = self.resolver.clone();

        let mut stmts = vec![];

        while self.cursor.node().kind() != "}" {
            stmts.push(self.lower_to_stmt()?);

            self.cursor.goto_next_sibling();
        }

        self.cursor.goto_parent();

        let resolver = mem::replace(&mut self.resolver, pre_resolver);

        Ok(Block {
            stmts,
            resolver,
            span,
        })
    }

    fn lower_to_stmt_kind(&mut self) -> anyhow::Result<StmtKind> {
        let node = self.cursor.node();
        trace!("Construct [StmtKind] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::COMPOUND_STATEMENT => StmtKind::Block(self.lower_to_block()?),
            constants::EXPRESSION_STATEMENT => StmtKind::Expr(self.lower_to_expr()?),
            constants::DECLARATION => StmtKind::Decl(self.lower_to_decl_stmt()?),
            constants::RETURN_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ret_expr = if self.cursor.node().kind().contains(";") {
                    None
                } else {
                    Some(self.lower_to_expr()?)
                };

                self.cursor.goto_parent();

                StmtKind::Ret(ret_expr)
            }
            constants::LABELED_STATEMENT => {
                self.cursor.goto_first_child();

                let ident = self.lower_to_ident()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let stmt = self.lower_to_stmt()?;

                self.cursor.goto_parent();

                let label_res = self
                    .label_resolver
                    .lookup_res(&ident.name)
                    .unwrap_or_else(|| {
                        self.label_resolver
                            .insert(ident.name, ())
                            .expect("Failed to insert label into resolver.")
                    });

                StmtKind::Label(label_res, Some(Box::new(stmt)))
            }
            constants::GOTO_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ident = self.lower_to_ident()?;

                self.cursor.goto_parent();

                let label_res = self
                    .label_resolver
                    .lookup_res(&ident.name)
                    .unwrap_or_else(|| {
                        self.label_resolver
                            .insert(ident.name, ())
                            .expect("Failed to insert label into resolver.")
                    });

                StmtKind::Goto(label_res)
            }
            constants::IF_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let cond_expr = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let body_stmt = self.lower_to_stmt()?;

                let else_stmt = if self.cursor.goto_next_sibling() {
                    self.cursor.goto_first_child();
                    self.cursor.goto_next_sibling();

                    let else_expr = self.lower_to_stmt()?;

                    self.cursor.goto_parent();

                    Some(else_expr)
                } else {
                    None
                };

                self.cursor.goto_parent();

                StmtKind::If(cond_expr, Box::new(body_stmt), else_stmt.map(Box::new))
            }
            constants::WHILE_STATEMENT => {
                /*
                    loop_start:
                        if (!$cond) goto loop_end;
                        $body;
                        goto loop_start:
                    loop_end:
                */

                let loop_start = format!("loop_start_{}_{}", span.lo, span.hi);
                let label_res_start = self.label_resolver.insert(loop_start.clone(), ())?;

                let loop_end = format!("loop_end_{}_{}", span.lo, span.hi);
                let label_res_end = self.label_resolver.insert(loop_end.clone(), ())?;

                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let cond_expr = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let body_stmt = self.lower_to_stmt()?;

                self.cursor.goto_parent();

                StmtKind::Block(Block {
                    stmts: vec![
                        Stmt {
                            kind: StmtKind::Label(label_res_start, None),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::If(
                                Expr {
                                    span: cond_expr.span,
                                    kind: ExprKind::Unary(UnOp::Not, Box::new(cond_expr)),
                                },
                                Box::new(Stmt {
                                    kind: StmtKind::Goto(label_res_end),
                                    span,
                                }),
                                None,
                            ),
                            span,
                        },
                        body_stmt,
                        Stmt {
                            kind: StmtKind::Goto(label_res_start),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::Label(label_res_end, None),
                            span,
                        },
                    ],
                    resolver: self.resolver.clone(),
                    span,
                })
            }
            constants::DO_STATEMENT => {
                todo!()
            }
            constants::FOR_STATEMENT => {
                todo!()
            }
            constants::CONTINUE_STATEMENT => {
                todo!()
            }
            constants::BREAK_STATEMENT => {
                todo!()
            }
            kind => bail!("Unsupported [StmtKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_to_stmt(&mut self) -> anyhow::Result<Stmt> {
        let node = self.cursor.node();
        trace!("Construct [Stmt] from node: {}", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let stmt_kind = self.lower_to_stmt_kind()?;

        Ok(Stmt {
            kind: stmt_kind,
            span,
        })
    }
}
