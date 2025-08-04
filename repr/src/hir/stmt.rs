#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::bail;
use log::trace;

use crate::hir::{resolver::SymbolKind, *};

use super::{
    constants,
    resolver::{Label, Symbol},
};

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Block(Block),
    Expr(Expr),
    Decl(Symbol),
    Ret(Option<Expr>),
    Label(Label, Option<Box<Stmt>>),
    Goto(Label),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_stmt(&mut self, node: Node) -> anyhow::Result<Stmt> {
        trace!("[HIR/Stmt] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let stmt_kind = self.lower_to_stmt_kind(node)?;

        Ok(Stmt {
            kind: stmt_kind,
            span,
        })
    }

    fn lower_to_stmt_kind(&mut self, node: Node) -> anyhow::Result<StmtKind> {
        trace!("[HIR/StmtKind] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::COMPOUND_STATEMENT => StmtKind::Block(self.lower_to_block(node)?),
            constants::EXPRESSION_STATEMENT => {
                StmtKind::Expr(self.lower_to_expr(node.child(0).unwrap())?)
            }
            constants::DECLARATION => {
                let local_decl = self.lower_to_local_decl(node)?;

                let symbol = self
                    .symbol_resolver
                    .insert_symbol(local_decl.ident.name.clone(), SymbolKind::Local(local_decl));

                StmtKind::Decl(symbol)
            }
            constants::RETURN_STATEMENT => {
                let ret_expr = if node.child_count() == 3 {
                    Some(self.lower_to_expr(node.child(1).unwrap())?)
                } else {
                    None
                };

                StmtKind::Ret(ret_expr)
            }
            constants::LABELED_STATEMENT => {
                let ident = self.lower_to_ident(node.child(0).unwrap())?;

                let stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                let label_res = self
                    .label_resolver
                    .get_res_by_name(&ident.name)
                    .unwrap_or_else(|| self.label_resolver.insert_symbol(ident.name, ()));

                StmtKind::Label(label_res, Some(Box::new(stmt)))
            }
            constants::GOTO_STATEMENT => {
                let ident = self.lower_to_ident(node.child(1).unwrap())?;

                let label_res = self
                    .label_resolver
                    .get_res_by_name(&ident.name)
                    .unwrap_or_else(|| self.label_resolver.insert_symbol(ident.name, ()));

                StmtKind::Goto(label_res)
            }
            constants::IF_STATEMENT => {
                let cond_expr = self.lower_to_expr(node.child(1).unwrap())?;

                let body_stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                let else_stmt = if let Some(node) = node.child(3) {
                    Some(self.lower_to_stmt(node.child(1).unwrap())?)
                } else {
                    None
                };

                StmtKind::If(cond_expr, Box::new(body_stmt), else_stmt.map(Box::new))
            }
            constants::WHILE_STATEMENT => {
                /*
                    loop_start:
                        if (!$cond) goto loop_end;
                        $body;
                        goto loop_start;
                    loop_end:
                */

                let loop_start = format!("loop_start_{}_{}", span.lo, span.hi);
                let loop_start_label = self.label_resolver.insert_symbol(loop_start.clone(), ());
                let saved_loop_start_label = self.loop_start_label;
                self.loop_start_label = Some(loop_start_label);

                let loop_end = format!("loop_end_{}_{}", span.lo, span.hi);
                let loop_end_label = self.label_resolver.insert_symbol(loop_end.clone(), ());
                let saved_loop_end_label = self.loop_end_label;
                self.loop_end_label = Some(loop_end_label);

                let cond_expr = self.lower_to_expr(node.child(1).unwrap())?;

                let body_stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                self.loop_start_label = saved_loop_start_label;
                self.loop_end_label = saved_loop_end_label;

                StmtKind::Block(Block {
                    symbol_resolver: self.symbol_resolver.clone(),
                    stmts: vec![
                        Stmt {
                            kind: StmtKind::Label(loop_start_label, None),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::If(
                                Expr {
                                    span: cond_expr.span,
                                    kind: ExprKind::Unary(UnOp::Not, Box::new(cond_expr)),
                                },
                                Box::new(Stmt {
                                    kind: StmtKind::Goto(loop_end_label),
                                    span,
                                }),
                                None,
                            ),
                            span,
                        },
                        body_stmt,
                        Stmt {
                            kind: StmtKind::Goto(loop_start_label),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::Label(loop_end_label, None),
                            span,
                        },
                    ],
                    span,
                })
            }
            constants::DO_STATEMENT => {
                /*
                    loop_start:
                        $body;
                        if ($cond) goto loop_start;
                    loop_end:
                */

                let loop_start = format!("loop_start_{}_{}", span.lo, span.hi);
                let loop_start_label = self.label_resolver.insert_symbol(loop_start.clone(), ());
                let saved_loop_start_label = self.loop_start_label;
                self.loop_start_label = Some(loop_start_label);

                let loop_end = format!("loop_end_{}_{}", span.lo, span.hi);
                let loop_end_label = self.label_resolver.insert_symbol(loop_end.clone(), ());
                let saved_loop_end_label = self.loop_end_label;
                self.loop_end_label = Some(loop_end_label);

                let body_stmt = self.lower_to_stmt(node.child(1).unwrap())?;

                let cond_expr = self.lower_to_expr(node.child(3).unwrap())?;

                self.loop_start_label = saved_loop_start_label;
                self.loop_end_label = saved_loop_end_label;

                StmtKind::Block(Block {
                    symbol_resolver: self.symbol_resolver.clone(),
                    stmts: vec![
                        Stmt {
                            kind: StmtKind::Label(loop_start_label, None),
                            span,
                        },
                        body_stmt,
                        Stmt {
                            kind: StmtKind::If(
                                cond_expr,
                                Box::new(Stmt {
                                    kind: StmtKind::Goto(loop_start_label),
                                    span,
                                }),
                                None,
                            ),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::Label(loop_end_label, None),
                            span,
                        },
                    ],
                    span,
                })
            }
            constants::FOR_STATEMENT => {
                /*
                    $decl
                    loop_start:
                        if (!$cond) goto loop_end;
                        $body;
                        $update;
                        goto loop_start;
                    loop_end:
                */

                let loop_start = format!("loop_start_{}_{}", span.lo, span.hi);
                let loop_start_label = self.label_resolver.insert_symbol(loop_start.clone(), ());
                let saved_loop_start_label = self.loop_start_label;
                self.loop_start_label = Some(loop_start_label);

                let loop_end = format!("loop_end_{}_{}", span.lo, span.hi);
                let loop_end_label = self.label_resolver.insert_symbol(loop_end.clone(), ());
                let saved_loop_end_label = self.loop_end_label;
                self.loop_end_label = Some(loop_end_label);

                let saved_symbol_resolver = self.symbol_resolver.clone();

                let decl_stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                let cond_expr = self.lower_to_expr(node.child(3).unwrap())?;

                let update_expr = self.lower_to_expr(node.child(5).unwrap())?;

                let body_stmt = self.lower_to_stmt(node.child(7).unwrap())?;

                self.loop_start_label = saved_loop_start_label;
                self.loop_end_label = saved_loop_end_label;

                let symbol_resolver =
                    mem::replace(&mut self.symbol_resolver, saved_symbol_resolver);

                StmtKind::Block(Block {
                    symbol_resolver,
                    stmts: vec![
                        decl_stmt,
                        Stmt {
                            kind: StmtKind::Label(loop_start_label, None),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::If(
                                Expr {
                                    span: cond_expr.span,
                                    kind: ExprKind::Unary(UnOp::Not, Box::new(cond_expr)),
                                },
                                Box::new(Stmt {
                                    kind: StmtKind::Goto(loop_end_label),
                                    span,
                                }),
                                None,
                            ),
                            span,
                        },
                        body_stmt,
                        Stmt {
                            kind: StmtKind::Expr(update_expr),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::Goto(loop_start_label),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::Label(loop_end_label, None),
                            span,
                        },
                    ],
                    span,
                })
            }
            constants::CONTINUE_STATEMENT => match self.loop_start_label {
                Some(loop_start_label) => StmtKind::Goto(loop_start_label),
                None => bail!("Continue statement outside of of loop body."),
            },
            constants::BREAK_STATEMENT => match self.loop_end_label {
                Some(loop_end_label) => StmtKind::Goto(loop_end_label),
                None => bail!("Break statement outside of of loop body."),
            },
            kind => bail!("Cannot lower '{kind}' to 'StmtKind'."),
        })
    }
}
