#![allow(clippy::missing_docs_in_private_items)]

//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

/// Contains datatypes used to represent the MIR.
mod datatypes;

pub use datatypes::*;

use std::collections::HashMap;

use la_arena::{Arena, RawIdx};
use smallvec::SmallVec;

use crate::hir::{self, PrimTyKind, Span, Ty, TyKind, resolver::Resolver};

impl<'mir> MirCtx<'mir> {
    pub fn new(resolver: &'mir Resolver, span: Span) -> Self {
        Self {
            body: Body {
                basic_blocks: Arena::new(),
                local_decls: Arena::new(),
                resolver,
                span,
            },
            local_map: HashMap::new(),
        }
    }

    pub(crate) fn alloc_bb(&mut self) -> BasicBlock {
        self.body.basic_blocks.alloc(BasicBlockData::default())
    }

    pub(crate) fn retrive_bb(&mut self, bb: BasicBlock) -> &mut BasicBlockData {
        &mut self.body.basic_blocks[bb]
    }

    pub(crate) fn alloc_local(
        &mut self,
        debug_ident: Option<String>,
        ty: &Ty,
        span: Span,
    ) -> Local {
        self.body.local_decls.alloc(LocalDecl {
            debug_ident,
            ty: ty.clone(),
            span,
        })
    }

    pub fn lower_to_mir(mut self, item: &'mir hir::Fn) -> anyhow::Result<Body<'mir>> {
        self.alloc_local(None, &item.sig.ty, item.body.span);

        for param in &item.sig.params {
            if let Some(res) = &param.res {
                let res_data = item.resolver.get_item(res);

                let local =
                    self.alloc_local(Some(res_data.ident.name.clone()), &param.ty, param.span);

                self.local_map.insert(*res, local);
            }
        }

        let bb = self.alloc_bb();
        _ = self.lower_to_basic_block(&item.body, bb);

        Ok(self.body)
    }

    pub(crate) fn lower_to_basic_block(
        &mut self,
        expr: &'mir hir::Expr,
        mut bb: BasicBlock,
    ) -> Option<BasicBlock> {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Block(block) => {
                let pre_resolver = self.body.resolver;
                self.body.resolver = &block.resolver;

                for stmt in &block.stmts {
                    bb = self.lower_stmt(stmt, bb);
                }

                self.body.resolver = pre_resolver;

                Some(bb)
            }
            hir::ExprKind::Lit(..) => todo!(),
            hir::ExprKind::Ret(expr) => {
                let rvalue = self.lower_to_rvalue(expr, bb);

                let bb_data = self.retrive_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        Place {
                            local: Local::from_raw(RawIdx::from_u32(0)),
                            projections: vec![],
                        },
                        rvalue,
                    ),
                    span,
                });

                bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });

                None
            }
            hir::ExprKind::Local(..) => todo!(),
            hir::ExprKind::Call(..) => todo!(),
            hir::ExprKind::Binary(..) => Some(bb),
            hir::ExprKind::Unary(..) => Some(bb),
            hir::ExprKind::If(cond_expr, body_expr, else_expr) => {
                let cond_rvalue = self.lower_to_rvalue(cond_expr, bb);

                let cond_local = self.alloc_local(
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        span: cond_expr.span,
                    },
                    cond_expr.span,
                );

                let cond_place = Place {
                    local: cond_local,
                    projections: vec![],
                };

                let body_bb = self.alloc_bb();
                let body_end_bb = self.lower_to_basic_block(body_expr, body_bb);

                let last_bb = match else_expr {
                    Some(else_expr) => {
                        let else_bb = self.alloc_bb();
                        let else_end_bb = self.lower_to_basic_block(else_expr, else_bb);

                        let last_bb = self.alloc_bb();

                        if let Some(body_last_bb) = body_end_bb {
                            self.retrive_bb(body_last_bb).terminator = Some(Terminator {
                                kind: TerminatorKind::Goto { bb: last_bb },
                                span,
                            });
                        }

                        if let Some(else_last_bb) = else_end_bb {
                            self.retrive_bb(else_last_bb).terminator = Some(Terminator {
                                kind: TerminatorKind::Goto { bb: last_bb },
                                span,
                            });
                        }

                        let bb_data = self.retrive_bb(bb);

                        bb_data.statements.push(Statement {
                            kind: StatementKind::Assign(cond_place.clone(), cond_rvalue),
                            span,
                        });

                        bb_data.terminator = Some(Terminator {
                            kind: TerminatorKind::SwitchInt {
                                discr: Operand::Place(cond_place),
                                targets: SwitchTargets {
                                    value: SmallVec::from_slice(&[0]),
                                    bbs: SmallVec::from_slice(&[else_bb, body_bb]),
                                },
                            },
                            span,
                        });

                        last_bb
                    }
                    None => {
                        let last_bb = self.alloc_bb();

                        if let Some(body_last_bb) = body_end_bb {
                            self.retrive_bb(body_last_bb).terminator = Some(Terminator {
                                kind: TerminatorKind::Goto { bb: last_bb },
                                span,
                            });
                        }

                        let bb_data = self.retrive_bb(bb);

                        bb_data.statements.push(Statement {
                            kind: StatementKind::Assign(cond_place.clone(), cond_rvalue),
                            span,
                        });

                        bb_data.terminator = Some(Terminator {
                            kind: TerminatorKind::SwitchInt {
                                discr: Operand::Place(cond_place),
                                targets: SwitchTargets {
                                    value: SmallVec::from_slice(&[0]),
                                    bbs: SmallVec::from_slice(&[last_bb, body_bb]),
                                },
                            },
                            span,
                        });

                        last_bb
                    }
                };

                Some(last_bb)
            }
            hir::ExprKind::Loop(..) => todo!(),
            hir::ExprKind::Break => todo!(),
            hir::ExprKind::Continue => todo!(),
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr, bb);

                let bb_data = self.retrive_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span,
                });

                None
            }
            hir::ExprKind::Field(..) => todo!(),
            hir::ExprKind::Index(..) => todo!(),
            hir::ExprKind::Cast(..) => todo!(),
            hir::ExprKind::Array(..) => todo!(),
            hir::ExprKind::AddrOf(..) => todo!(),
            hir::ExprKind::Comma(..) => todo!(),
            hir::ExprKind::Sizeof(..) => todo!(),
        }
    }

    pub(crate) fn lower_stmt(&mut self, stmt: &'mir hir::Stmt, bb: BasicBlock) -> BasicBlock {
        let span = stmt.span;

        match &stmt.kind {
            hir::StmtKind::Decl(decl_stmt) => {
                let init_rvalue = decl_stmt
                    .init
                    .as_ref()
                    .map(|init_expr| self.lower_to_rvalue(init_expr, bb));

                let res_data = self.body.resolver.get_item(&decl_stmt.res);

                let local = self.alloc_local(
                    Some(res_data.ident.name.clone()),
                    &decl_stmt.ty,
                    decl_stmt.span,
                );

                self.local_map.insert(decl_stmt.res, local);

                let bb_data = self.retrive_bb(bb);

                if let Some(init_rvalue) = init_rvalue {
                    bb_data.statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local,
                                projections: vec![],
                            },
                            init_rvalue,
                        ),
                        span,
                    });
                }

                bb
            }
            hir::StmtKind::Expr(expr) => self.lower_to_basic_block(expr, bb).unwrap_or(bb),
            hir::StmtKind::Semi(expr) => self.lower_to_basic_block(expr, bb).unwrap_or(bb),
        }
    }

    pub(crate) fn lower_to_rvalue(&mut self, expr: &'mir hir::Expr, bb: BasicBlock) -> Rvalue {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Local(_) | hir::ExprKind::Lit(_) => {
                Rvalue::Use(self.lower_to_operand(expr, bb))
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb);
                let right_operand = self.lower_to_operand(right_expr, bb);

                Rvalue::BinaryOp(*bin_op, left_operand, right_operand)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb);

                Rvalue::UnaryOp(*un_op, operand)
            }
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr, bb);

                let bb_data = self.retrive_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span,
                });

                Rvalue::Use(Operand::Place(place))
            }
            hir::ExprKind::Call(expr, exprs) => {
                let operand = self.lower_to_operand(expr, bb);

                let arguments = exprs
                    .iter()
                    .map(|expr| self.lower_to_operand(expr, bb))
                    .collect();

                Rvalue::Call(operand, arguments)
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn lower_to_operand(&mut self, expr: &'mir hir::Expr, bb: BasicBlock) -> Operand {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Lit(lit) => Operand::Const(Const::Lit(lit.clone())),
            hir::ExprKind::Local(res) => {
                let Some(local) = self.local_map.get(res) else {
                    return Operand::Const(Const::Fn(*res));
                };

                Operand::Place(Place {
                    local: *local,
                    projections: vec![],
                })
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb);
                let right_operand = self.lower_to_operand(right_expr, bb);

                let local = self.alloc_local(
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        span,
                    },
                    span,
                );

                let place = Place {
                    local,
                    projections: vec![],
                };

                let bb_data = self.retrive_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        place.clone(),
                        Rvalue::BinaryOp(*bin_op, left_operand, right_operand),
                    ),
                    span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb);

                let local = self.alloc_local(
                    None,
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        span,
                    },
                    span,
                );

                let place = Place {
                    local,
                    projections: vec![],
                };

                let bb_data = self.retrive_bb(bb);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), Rvalue::UnaryOp(*un_op, operand)),
                    span,
                });

                Operand::Place(place)
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn lower_to_place(&mut self, expr: &'mir hir::Expr) -> Place {
        match &expr.kind {
            hir::ExprKind::Local(res) => {
                let local = self.local_map.get(res).unwrap();

                Place {
                    local: *local,
                    projections: vec![],
                }
            }
            _ => unreachable!(),
        }
    }
}
