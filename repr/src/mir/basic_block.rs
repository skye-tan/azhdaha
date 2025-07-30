#![allow(clippy::missing_docs_in_private_items)]

use la_arena::RawIdx;
use smallvec::SmallVec;

use crate::{
    hir::{self, PrimTyKind, Ty, TyKind},
    mir::{MirCtx, datatypes::*},
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn lower_to_bb(&mut self, stmt: &'mir hir::Stmt, mut bb: BasicBlock) -> BasicBlock {
        let span = stmt.span;

        match &stmt.kind {
            hir::StmtKind::Block(block) => {
                let pre_resolver = self.body.symbol_resolver;
                self.body.symbol_resolver = &block.symbol_resolver;

                for stmt in &block.stmts {
                    bb = self.lower_to_bb(stmt, bb);
                }

                self.body.symbol_resolver = pre_resolver;

                bb
            }
            hir::StmtKind::Expr(expr) => {
                _ = self.lower_to_rvalue(expr, bb);

                bb
            }
            hir::StmtKind::Decl(symbol) => {
                let hir::LocalDecl {
                    storage,
                    ident,
                    ty,
                    init,
                    span,
                } = match self.body.symbol_resolver.get_data_by_res(symbol) {
                    hir::resolver::SymbolKind::Local(local_decl) => local_decl,
                    _ => unreachable!(),
                };

                let init_rvalue = init
                    .as_ref()
                    .map(|init_expr| self.lower_to_rvalue(init_expr, bb));

                let local = self.alloc_local(Some(ident.name.clone()), storage.clone(), ty, *span);

                self.local_map.insert(*symbol, local);

                if let Some(init_rvalue) = init_rvalue {
                    self.retrieve_bb(bb).statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local,
                                projections: vec![],
                            },
                            init_rvalue,
                        ),
                        span: *span,
                    });
                }

                bb
            }
            hir::StmtKind::Ret(ret_expr) => {
                if let Some(ret_expr) = ret_expr {
                    let rvalue = self.lower_to_rvalue(ret_expr, bb);

                    self.retrieve_bb(bb).statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local: Local::from_raw(RawIdx::from_u32(0)),
                                projections: vec![],
                            },
                            rvalue,
                        ),
                        span,
                    });
                }

                self.retrieve_bb(bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });

                bb
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

                self.retrieve_bb(bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                if let Some(stmt) = stmt {
                    next_bb = self.lower_to_bb(stmt, next_bb);
                }

                next_bb
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

                self.retrieve_bb(bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                // TODO: Currently a new basic block is created after each "goto" statement which
                // may contain unreachable code. I might want to consider generating a warning for
                // the non-empty variant of these basic blocks in the future or ignore them.
                //
                self.alloc_bb()
            }
            hir::StmtKind::If(cond_expr, body_stmt, else_stmt) => {
                let cond_rvalue = self.lower_to_rvalue(cond_expr, bb);

                let cond_local = self.alloc_local(
                    None,
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        is_linear: false,
                        quals: vec![],
                        span: cond_expr.span,
                    },
                    cond_expr.span,
                );

                let cond_place = Place {
                    local: cond_local,
                    projections: vec![],
                };

                let body_bb = self.alloc_bb();
                let body_last_bb = self.lower_to_bb(body_stmt, body_bb);

                let next_bb = self.alloc_bb();

                self.retrieve_bb(body_last_bb).terminator = Some(Terminator {
                    kind: TerminatorKind::Goto { bb: next_bb },
                    span,
                });

                let else_bb = if let Some(else_stmt) = else_stmt {
                    let else_bb = self.alloc_bb();
                    let else_last_bb = self.lower_to_bb(else_stmt, else_bb);

                    self.retrieve_bb(else_last_bb).terminator = Some(Terminator {
                        kind: TerminatorKind::Goto { bb: next_bb },
                        span,
                    });

                    else_bb
                } else {
                    next_bb
                };

                let bb_data = self.retrieve_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(cond_place.clone(), cond_rvalue),
                    span,
                });

                bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::SwitchInt {
                        discr: Operand::Place(cond_place),
                        targets: SwitchTargets {
                            value: SmallVec::from_slice(&[1]),
                            bbs: SmallVec::from_slice(&[body_bb, else_bb]),
                        },
                    },
                    span,
                });

                next_bb
            }
        }
    }
}
