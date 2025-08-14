#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{self},
    mir::{MirCtx, RETURN_LOCAL, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_bb(&mut self, stmt: &'mir hir::Stmt, bb: &mut BasicBlock) {
        let span = stmt.span;

        match &stmt.kind {
            hir::StmtKind::Block(block) => {
                let saved_symbol_resolver = self.body.symbol_resolver;

                self.body.symbol_resolver = &block.symbol_resolver;

                for stmt in &block.stmts {
                    self.lower_to_bb(stmt, bb);
                }

                if self.has_inner_symbol_resolver {
                    self.body.symbol_resolver = saved_symbol_resolver;
                } else {
                    self.has_inner_symbol_resolver = true;
                }
            }
            hir::StmtKind::Expr(expr) => {
                let rvalue = self.lower_to_rvalue(expr, bb, span);

                if let Rvalue::Call(operand, operands) = rvalue {
                    self.retrieve_bb(*bb).statements.push(Statement {
                        kind: StatementKind::Call(operand, operands),
                        span,
                    })
                }
            }
            hir::StmtKind::Decl(symbol) => {
                let symbol_kind = self.body.symbol_resolver.get_data_by_res(symbol);

                let hir::LocalDecl {
                    storage,
                    ident,
                    ty,
                    init,
                    span: _,
                } = match symbol_kind {
                    hir::resolver::SymbolKind::Local(local_decl) => local_decl,
                    _ => unreachable!(),
                };

                let init_rvalue = init
                    .as_ref()
                    .map(|init_expr| self.lower_to_rvalue(init_expr, bb, span));

                let local = self.alloc_local(Some(ident.name.clone()), storage.clone(), ty, span);

                self.local_map.insert(*symbol, local);

                if let Some(init_rvalue) = init_rvalue {
                    self.retrieve_bb(*bb).statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local,
                                projections: vec![],
                                span: ident.span,
                            },
                            init_rvalue,
                        ),
                        span,
                    });
                }
            }
            hir::StmtKind::Ret(ret_expr) => {
                if let Some(ret_expr) = ret_expr {
                    let ret_rvalue = self.lower_to_rvalue(ret_expr, bb, span);

                    self.retrieve_bb(*bb).statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local: RETURN_LOCAL,
                                projections: vec![],
                                span,
                            },
                            ret_rvalue,
                        ),
                        span,
                    });
                }

                self.retrieve_bb(*bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });
            }
            hir::StmtKind::Label(label_idx, stmt) => {
                let mut next_bb = match self.bb_map.get(label_idx) {
                    Some(next_bb) => *next_bb,
                    None => {
                        let next_bb = self.alloc_bb();

                        self.bb_map.insert(*label_idx, next_bb);

                        next_bb
                    }
                };

                self.retrieve_bb(*bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                if let Some(stmt) = stmt {
                    self.lower_to_bb(stmt, &mut next_bb);
                }

                bb.set(next_bb);
            }
            hir::StmtKind::Goto(label_idx) => {
                let next_bb = match self.bb_map.get(label_idx) {
                    Some(next_bb) => *next_bb,
                    None => {
                        let next_bb = self.alloc_bb();

                        self.bb_map.insert(*label_idx, next_bb);

                        next_bb
                    }
                };

                self.retrieve_bb(*bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                // Currently a new basic block is created after each "goto" statement which may
                // contain unreachable code. I might want to consider generating a warning for
                // the non-empty variant of these basic-blocks in the future or ignore them.
                bb.set(self.alloc_bb());
            }
            hir::StmtKind::If(cond_expr, body_stmt, else_stmt) => {
                let cond_rvalue = self.lower_to_rvalue(cond_expr, bb, span);
                let cond_place = self.store_in_temp_place(cond_rvalue, bb, span);

                let body_bb = self.alloc_bb();
                let mut body_last_bb = body_bb;
                self.lower_to_bb(body_stmt, &mut body_last_bb);

                let next_bb = self.alloc_bb();

                self.retrieve_bb(body_last_bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                let else_bb = if let Some(else_stmt) = else_stmt {
                    let else_bb = self.alloc_bb();
                    let mut else_last_bb = else_bb;
                    self.lower_to_bb(else_stmt, &mut else_last_bb);

                    self.retrieve_bb(else_last_bb).terminator = Some(Terminator {
                        kind: TerminatorKind::Goto { bb: next_bb },
                        span,
                    });

                    else_bb
                } else {
                    next_bb
                };

                self.retrieve_bb(*bb).terminator = Some(Terminator {
                    kind: TerminatorKind::SwitchInt {
                        discr: Operand::Place(cond_place),
                        targets: [body_bb, else_bb],
                    },
                    span,
                });

                bb.set(next_bb);
            }
            hir::StmtKind::TyDef(..) => (),
        }
    }
}
