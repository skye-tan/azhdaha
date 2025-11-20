#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use log::trace;

use crate::hir::{resolver::SymbolKind, *};

use super::{
    constants,
    resolver::{Label, Symbol},
};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    Block(Block),
    Expr(Expr),
    Decl(Vec<Symbol>),
    Ret(Option<Expr>),
    Label(Label, Option<Box<Stmt>>),
    Goto(Label),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Noop,
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
                let var_decl_list = self.lower_to_var_decl_list(node)?;

                let mut symbols = vec![];

                for var_decl in var_decl_list {
                    let symbol = self
                        .symbol_resolver
                        .insert_symbol(var_decl.ident.name.clone(), SymbolKind::Var(var_decl));

                    symbols.push(symbol);
                }

                StmtKind::Decl(symbols)
            }
            constants::RETURN_STATEMENT => {
                let ret_expr = if node.child_count() == 3 {
                    Some(
                        self.lower_to_expr_with_expected_type(
                            node.child(1).unwrap(),
                            self.return_ty
                                .clone()
                                .expect("Return type is not filled at the start"),
                        )?,
                    )
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
                let cond_expr = self.lower_to_cond_expr(node.child(1).unwrap())?;

                let body_stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                let else_stmt = if let Some(node) = node.child(3) {
                    Some(self.lower_to_stmt(node.child(1).unwrap())?)
                } else {
                    None
                };

                StmtKind::If(cond_expr, Box::new(body_stmt), else_stmt.map(Box::new))
            }
            constants::SWITCH_STATEMENT => {
                let cond_expr = self.lower_to_expr(node.child(1).unwrap())?;

                let saved_switch_cond = self.switch_cond.replace(SwitchData::default());

                let switch_end_label = self.label_resolver.insert_unnamed_symbol(());
                let saved_end_label = self.end_label;
                self.end_label = Some(switch_end_label);

                let body_stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                let my_switch_data = self.switch_cond.take().unwrap();
                self.switch_cond = saved_switch_cond;
                self.end_label = saved_end_label;

                let ty = cond_expr.ty.clone();

                let cond_storage =
                    self.symbol_resolver
                        .insert_unnamed_symbol(SymbolKind::Var(VarDecl {
                            storage: None,
                            ident: Ident {
                                name: "_switch_cond".to_owned(),
                                span,
                            },
                            ty: ty.clone(),
                            init: Some(cond_expr),
                            span,
                        }));

                let mut stmts = vec![Stmt {
                    kind: StmtKind::Decl(vec![cond_storage]),
                    span,
                }];

                for (cond, label) in my_switch_data.cases {
                    stmts.push(Stmt {
                        kind: StmtKind::If(
                            Expr {
                                kind: ExprKind::Binary(
                                    BinOp::Eq,
                                    Box::new(Expr {
                                        kind: ExprKind::Local(cond_storage),
                                        ty: ty.clone(),
                                        span,
                                    }),
                                    Box::new(Expr {
                                        kind: ExprKind::Lit(Lit {
                                            kind: LitKind::Int(cond as i128),
                                            span,
                                        }),
                                        ty: ty.clone(),
                                        span,
                                    }),
                                ),
                                ty: Ty {
                                    kind: TyKind::PrimTy(PrimTyKind::Bool),
                                    is_linear: false,
                                    quals: vec![],
                                    span,
                                },
                                span,
                            },
                            Box::new(Stmt {
                                kind: StmtKind::Goto(label),
                                span,
                            }),
                            None,
                        ),
                        span,
                    });
                }

                stmts.push(Stmt {
                    kind: StmtKind::Goto(my_switch_data.default_case.unwrap_or(switch_end_label)),
                    span,
                });
                stmts.push(body_stmt);
                stmts.push(Stmt {
                    kind: StmtKind::Label(switch_end_label, None),
                    span,
                });

                StmtKind::Block(Block { stmts, span })
            }
            constants::CASE_STATEMENT => {
                let Some(mut switch_data) = self.switch_cond.take() else {
                    bail!("Case statement outside of switch body.")
                };

                let label = self.label_resolver.insert_unnamed_symbol(());

                let stmt_child_index = match node.child(0).unwrap().kind() {
                    constants::CASE => {
                        let case_value =
                            self.const_eval_enum_value(node.child_by_field_name("value").unwrap())?;
                        switch_data.cases.push((case_value, label));
                        3
                    }
                    constants::DEFAULT => {
                        if switch_data.default_case.is_some() {
                            bail!("Duplicate default label in switch case.");
                        }
                        switch_data.default_case = Some(label);
                        2
                    }
                    kind => bail!("Unknown keyword '{kind}' in switch statement."),
                };

                let mut stmts = vec![];

                let mut cursor = node.walk();

                for child in node.children(&mut cursor).skip(stmt_child_index) {
                    stmts.push(self.lower_to_stmt(child)?);
                }
                self.switch_cond = Some(switch_data);
                StmtKind::Label(
                    label,
                    Some(Box::new(Stmt {
                        kind: StmtKind::Block(Block { stmts, span }),
                        span,
                    })),
                )
            }
            constants::WHILE_STATEMENT => {
                /*
                    loop_start:
                        if (!$cond) goto loop_end;
                        $body;
                        goto loop_start;
                    loop_end:
                */

                let cond_expr = self.lower_to_cond_expr(node.child(1).unwrap())?;

                let loop_start_label = self.label_resolver.insert_unnamed_symbol(());
                let saved_start_label = self.start_label;
                self.start_label = Some(loop_start_label);

                let loop_end_label = self.label_resolver.insert_unnamed_symbol(());
                let saved_end_label = self.end_label;
                self.end_label = Some(loop_end_label);

                let body_stmt = self.lower_to_stmt(node.child(2).unwrap())?;

                self.start_label = saved_start_label;
                self.end_label = saved_end_label;

                StmtKind::Block(Block {
                    stmts: vec![
                        Stmt {
                            kind: StmtKind::Label(loop_start_label, None),
                            span,
                        },
                        Stmt {
                            kind: StmtKind::If(
                                Expr {
                                    span: cond_expr.span,
                                    ty: cond_expr.ty.clone(),
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

                let loop_start_label = self.label_resolver.insert_unnamed_symbol(());
                let saved_start_label = self.start_label;
                self.start_label = Some(loop_start_label);

                let loop_end_label = self.label_resolver.insert_unnamed_symbol(());
                let saved_end_label = self.end_label;
                self.end_label = Some(loop_end_label);

                let body_stmt = self.lower_to_stmt(node.child(1).unwrap())?;

                let cond_expr = self.lower_to_cond_expr(node.child(3).unwrap())?;

                self.start_label = saved_start_label;
                self.end_label = saved_end_label;

                StmtKind::Block(Block {
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

                let decl_stmt = match node.child_by_field_name("initializer") {
                    Some(init) => {
                        if init.kind() == constants::DECLARATION {
                            self.lower_to_stmt(init)?
                        } else {
                            Stmt {
                                kind: StmtKind::Expr(self.lower_to_cond_expr(init)?),
                                span,
                            }
                        }
                    }
                    None => Stmt {
                        kind: StmtKind::Noop,
                        span,
                    },
                };

                let cond_expr = match node.child_by_field_name("condition") {
                    Some(node) => self.lower_to_cond_expr(node)?,
                    None => Expr {
                        kind: ExprKind::Lit(Lit {
                            kind: LitKind::Int(1),
                            span,
                        }),
                        ty: Ty {
                            kind: TyKind::PrimTy(PrimTyKind::Bool),
                            is_linear: false,
                            quals: vec![],
                            span,
                        },
                        span,
                    },
                };

                let update_expr = match node.child_by_field_name("update") {
                    Some(update) => self.lower_to_expr(update)?,
                    None => Expr {
                        kind: ExprKind::Empty,
                        ty: Ty {
                            kind: TyKind::PrimTy(PrimTyKind::Void),
                            is_linear: false,
                            quals: vec![],
                            span,
                        },
                        span,
                    },
                };

                let loop_start_label = self.label_resolver.insert_unnamed_symbol(());
                let loop_continue_label = self.label_resolver.insert_unnamed_symbol(());

                let saved_start_label = self.start_label;
                self.start_label = Some(loop_continue_label);

                let loop_end_label = self.label_resolver.insert_unnamed_symbol(());
                let saved_end_label = self.end_label;
                self.end_label = Some(loop_end_label);

                let saved_symbol_resolver = self.symbol_resolver.open_new_scope();

                let body_stmt = self.lower_to_stmt(node.child_by_field_name("body").unwrap())?;

                self.start_label = saved_start_label;
                self.end_label = saved_end_label;

                self.symbol_resolver
                    .restore_prev_scope(saved_symbol_resolver);

                StmtKind::Block(Block {
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
                                    ty: cond_expr.ty.clone(),
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
                            kind: StmtKind::Label(loop_continue_label, None),
                            span,
                        },
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
            constants::CONTINUE_STATEMENT => match self.start_label {
                Some(loop_start_label) => StmtKind::Goto(loop_start_label),
                None => bail!("Continue statement outside of loop body."),
            },
            constants::BREAK_STATEMENT => match self.end_label {
                Some(loop_end_label) => StmtKind::Goto(loop_end_label),
                None => bail!("Break statement outside of loop or switch body."),
            },
            constants::TYPE_DEFINITION => {
                let var_decl = self.lower_to_var_decl(node)?;

                self.symbol_resolver
                    .insert_symbol(var_decl.ident.name, SymbolKind::TyDef(var_decl.ty));

                StmtKind::Noop
            }
            constants::STRUCT_SPECIFIER
            | constants::UNION_SPECIFIER
            | constants::ENUM_SPECIFIER => {
                self.lower_struct_or_union_or_enum(node)?;

                StmtKind::Noop
            }
            constants::SEMICOLON => StmtKind::Noop,
            kind => bail!("Cannot lower '{kind}' to 'StmtKind'."),
        })
    }
}
