#![allow(clippy::missing_docs_in_private_items)]

use anyhow::{Context, bail};
use log::trace;

use crate::hir::*;

use super::{constants, resolver::Symbol};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Lit(Lit),
    Local(Symbol),
    Call(Box<Expr>, Vec<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Field(Box<Expr>, Ident),
    PtrOffset(Box<Expr>, Box<Expr>),
    PtrDiff(Box<Expr>, Box<Expr>),
    Cast(Box<Expr>),
    Array(Vec<Expr>),
    Comma(Vec<Expr>),
    Sizeof(Sizeof),
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    GnuBlock(Block),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Sizeof {
    pub kind: SizeofKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum SizeofKind {
    Ty(Ty),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LitKind {
    Str(String),
    Char(char),
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Or,
    And,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Not,
    Neg,
    Com,
    Pos,
    AddrOf,
    Deref,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_expr(&mut self, node: Node) -> anyhow::Result<Expr> {
        trace!("[HIR/Expr] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let (kind, ty) = self.lower_to_expr_kind(node)?;

        Ok(Expr { kind, ty, span })
    }

    fn lower_un_op(
        &mut self,
        expr: Expr,
        un_op: UnOp,
        span: Span,
    ) -> anyhow::Result<(ExprKind, Ty)> {
        let ty = match un_op {
            UnOp::Not | UnOp::Neg | UnOp::Com | UnOp::Pos => expr.ty.clone(),
            UnOp::AddrOf => Ty {
                kind: TyKind::Ptr {
                    kind: Box::new(expr.ty.kind.clone()),
                    quals: vec![],
                },
                is_linear: false,
                quals: vec![],
                span,
            },
            UnOp::Deref => {
                let TyKind::Ptr { kind, quals: _ } = &expr.ty.kind else {
                    bail!("Type error: dereference of non-ptr type");
                };
                Ty {
                    kind: *kind.clone(),
                    is_linear: false,
                    quals: vec![],
                    span,
                }
            }
        };

        // Ignore [`UnOp::Pos`] because it has no effects.
        Ok(match un_op {
            UnOp::Pos => (expr.kind, expr.ty),
            _ => (ExprKind::Unary(un_op, Box::new(expr)), ty),
        })
    }

    fn lower_bin_op(
        &mut self,
        mut lhs: Expr,
        mut rhs: Expr,
        bin_op: BinOp,
        span: Span,
    ) -> anyhow::Result<(ExprKind, Ty)> {
        self.array_to_pointer_decay_if_array(&mut lhs);
        self.array_to_pointer_decay_if_array(&mut rhs);

        'check_pointers: {
            if bin_op == BinOp::Add {
                let lhs_is_ptr = lhs.ty.kind.is_ptr();
                let rhs_is_ptr = rhs.ty.kind.is_ptr();

                match (lhs_is_ptr, rhs_is_ptr) {
                    (true, true) => bail!("Type error: adding two pointers"),
                    (true, false) => (),
                    (false, true) => {
                        std::mem::swap(&mut lhs, &mut rhs);
                    }
                    (false, false) => break 'check_pointers,
                }

                let ty = lhs.ty.clone();
                return Ok((ExprKind::PtrOffset(Box::new(lhs), Box::new(rhs)), ty));
            }
            if bin_op == BinOp::Sub {
                let lhs_is_ptr = lhs.ty.kind.is_ptr();
                let rhs_is_ptr = rhs.ty.kind.is_ptr();

                match (lhs_is_ptr, rhs_is_ptr) {
                    (true, true) => {
                        let ty = Ty {
                            kind: TyKind::PrimTy(PrimTyKind::Int),
                            is_linear: false,
                            quals: vec![],
                            span,
                        };
                        return Ok((ExprKind::PtrDiff(Box::new(lhs), Box::new(rhs)), ty));
                    }
                    (true, false) => lhs.ty.clone(),
                    (false, true) => {
                        std::mem::swap(&mut lhs, &mut rhs);
                        lhs.ty.clone()
                    }
                    (false, false) => break 'check_pointers,
                };

                let ty = lhs.ty.clone();
                let rhs = Expr {
                    ty: rhs.ty.clone(),
                    span: rhs.span,
                    kind: ExprKind::Unary(UnOp::Neg, Box::new(rhs)),
                };

                return Ok((ExprKind::PtrOffset(Box::new(lhs), Box::new(rhs)), ty));
            }
        }

        let ty = lhs.ty.clone(); // TODO: Care about casts

        Ok((ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs)), ty))
    }

    fn array_to_pointer_decay_if_array(&mut self, expr: &mut Expr) {
        if !expr.ty.kind.is_array() {
            return;
        }
        *expr = self.array_to_pointer_decay(expr.clone());
    }

    fn array_to_pointer_decay(&mut self, expr: Expr) -> Expr {
        let TyKind::Array { kind, size: _ } = &expr.ty.kind else {
            panic!("Expr is not array");
        };

        let ty = Ty {
            kind: TyKind::Ptr {
                kind: kind.clone(),
                quals: vec![],
            },
            is_linear: false,
            quals: vec![],
            span: expr.span,
        };

        Expr {
            span: expr.span,
            kind: ExprKind::Cast(Box::new(expr)),
            ty,
        }
    }

    fn lower_to_expr_kind(&mut self, node: Node) -> anyhow::Result<(ExprKind, Ty)> {
        trace!("[HIR/ExprKind] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::IDENTIFIER => {
                let ident = self.lower_to_ident(node)?;

                let symbol = self
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .context(format!("Use of undefined identifier '{}'.", &ident.name))?;

                let ty = self.symbol_resolver.arena[symbol].ty();

                (ExprKind::Local(symbol), ty)
            }
            constants::CALL_EXPRESSION => {
                let mut cursor = node.walk();
                cursor.goto_first_child();

                let path = self.lower_to_expr(cursor.node())?;

                let mut arguments = vec![];

                cursor.goto_next_sibling();
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                while cursor.node().kind() != ")" {
                    arguments.push(self.lower_to_expr(cursor.node())?);

                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();
                }

                let ty = path.ty.clone(); // TODO: wrong

                (ExprKind::Call(Box::new(path), arguments), ty)
            }
            constants::BINARY_EXPRESSION => {
                let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                let bin_op = self
                    .lower_to_bin_op(node.child(1).unwrap())?
                    .expect("Assignment isn't valid here");

                let rhs = self.lower_to_expr(node.child(2).unwrap())?;

                self.lower_bin_op(lhs, rhs, bin_op, span)?
            }
            constants::UPDATE_EXPRESSION => {
                let (lhs, bin_op) = if let Ok(bin_op) = self.lower_to_bin_op(node.child(1).unwrap())
                {
                    let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                    (lhs, bin_op)
                } else {
                    let bin_op = self.lower_to_bin_op(node.child(0).unwrap())?;

                    let lhs = self.lower_to_expr(node.child(1).unwrap())?;

                    (lhs, bin_op)
                };

                let rhs = Expr {
                    kind: ExprKind::Lit(Lit {
                        kind: LitKind::Int(1),
                        span,
                    }),
                    span,
                    ty: lhs.ty.clone(), // TODO: probably wrong
                };

                let ty = lhs.ty.clone();
                let bin_op = bin_op.expect("Assignment isn't valid operator for update?");

                (
                    ExprKind::Assign(
                        Box::new(lhs.clone()),
                        Box::new(Expr {
                            kind: ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs)),
                            span,
                            ty: ty.clone(),
                        }),
                    ),
                    ty,
                )
            }
            constants::UNARY_EXPRESSION | constants::POINTER_EXPRESSION => {
                let un_op = self.lower_to_un_op(node.child(0).unwrap())?;
                let expr = self.lower_to_expr(node.child(1).unwrap())?;

                self.lower_un_op(expr, un_op, span)?
            }
            constants::PARENTHESIZED_EXPRESSION => {
                let child = node.child(1).unwrap();
                if child.kind() == constants::COMPOUND_STATEMENT {
                    let block = self.lower_to_block(child)?;
                    let StmtKind::Expr(last_expr) = &block.stmts.last().unwrap().kind else {
                        bail!("Invalid gnu statement block");
                    };
                    let ty = last_expr.ty.clone();
                    (ExprKind::GnuBlock(block), ty)
                } else {
                    self.lower_to_expr_kind(child)?
                }
            }
            constants::ASSIGNMENT_EXPRESSION => {
                let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                let bin_op = self.lower_to_bin_op(node.child(1).unwrap())?;

                let rhs = self.lower_to_expr(node.child(2).unwrap())?;

                let ty = rhs.ty.clone();

                (
                    match bin_op {
                        None => ExprKind::Assign(Box::new(lhs), Box::new(rhs)),
                        Some(bin_op) => ExprKind::Assign(
                            Box::new(lhs.clone()),
                            Box::new(Expr {
                                kind: ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs)),
                                span,
                                ty: ty.clone(),
                            }),
                        ),
                    },
                    ty,
                )
            }
            constants::FIELD_EXPRESSION => {
                let target = self.lower_to_expr(node.child(0).unwrap())?;

                let field = self.lower_to_ident(node.child(2).unwrap())?;

                let ty = target.ty.clone(); // TODO: pure garbage

                (ExprKind::Field(Box::new(target), field), ty)
            }
            constants::SUBSCRIPT_EXPRESSION => {
                let target = self.lower_to_expr(node.child(0).unwrap())?;

                let index = self.lower_to_expr(node.child(2).unwrap())?;

                let a_plus_i = {
                    let (kind, ty) = self.lower_bin_op(target, index, BinOp::Add, span)?;
                    Expr { kind, ty, span }
                };
                self.lower_un_op(a_plus_i, UnOp::Deref, span)?
            }
            constants::CAST_EXPRESSION => {
                let cast_node = node.child(1).unwrap();

                let ty_kind = self.lower_to_ty_kind(cast_node, None)?;

                let target = self.lower_to_expr(node.child(3).unwrap())?;

                let ty = Ty {
                    kind: ty_kind.clone(),
                    is_linear: false, // TODO: who knows?
                    quals: vec![],
                    span,
                };

                (ExprKind::Cast(Box::new(target)), ty)
            }
            constants::INITIALIZER_LIST => {
                let mut elements = vec![];

                let mut cursor = node.walk();
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                loop {
                    elements.push(self.lower_to_expr(cursor.node())?);

                    cursor.goto_next_sibling();
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                let ty = Ty {
                    kind: TyKind::Array {
                        kind: Box::new(TyKind::PrimTy(PrimTyKind::Void)), // TODO: non sense
                        size: None, // TODO: Why this has type Expr?
                    },
                    is_linear: false,
                    quals: vec![],
                    span,
                };

                (ExprKind::Array(elements), ty)
            }
            constants::COMMA_EXPRESSION => {
                let mut exprs = vec![];

                let mut cursor = node.walk();
                cursor.goto_first_child();

                loop {
                    exprs.push(self.lower_to_expr(cursor.node())?);

                    cursor.goto_next_sibling();
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                let ty = exprs.last().unwrap().ty.clone();

                (ExprKind::Comma(exprs), ty)
            }
            constants::CONDITIONAL_EXPRESSION => {
                let cond_expr = self.lower_to_expr(node.child(0).unwrap())?;

                let body_expr = self.lower_to_expr(node.child(2).unwrap())?;

                let else_expr = self.lower_to_expr(node.child(4).unwrap())?;

                let ty = body_expr.ty.clone(); // TODO: handle casts

                (
                    ExprKind::Cond(
                        Box::new(cond_expr),
                        Box::new(body_expr),
                        Box::new(else_expr),
                    ),
                    ty,
                )
            }
            constants::SIZEOF_EXPRESSION => (
                ExprKind::Sizeof(self.lower_to_sizeof(node)?),
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Int),
                    is_linear: false,
                    quals: vec![],
                    span,
                },
            ),
            constants::SEMICOLON_EXPRESSION => (
                ExprKind::Empty,
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Void),
                    is_linear: false,
                    quals: vec![],
                    span,
                },
            ),
            kind if kind.contains(constants::LITERAL) => (
                ExprKind::Lit(self.lower_to_lit(node)?),
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Int), // TODO: handle other literals
                    is_linear: false,
                    quals: vec![],
                    span,
                },
            ),
            kind => bail!("Cannot lower '{kind}' to 'ExprKind'."),
        })
    }

    fn lower_to_sizeof(&mut self, node: Node) -> anyhow::Result<Sizeof> {
        trace!("[HIR/SizeOf] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let kind = self.lower_to_sizeof_kind(node)?;

        Ok(Sizeof { kind, span })
    }

    fn lower_to_sizeof_kind(&mut self, node: Node) -> anyhow::Result<SizeofKind> {
        trace!("[HIR/SizeofKind] Lowering '{}'", node.kind());

        let sizeof_kind = 'size_of: {
            let child = node.child(1).unwrap();
            if child.kind() == constants::PARENTHESIZED_EXPRESSION {
                break 'size_of SizeofKind::Expr(Box::new(self.lower_to_expr(child)?));
            }

            let child = node.child(2).unwrap();
            if child.kind() == constants::TYPE_DESCRIPTOR {
                break 'size_of SizeofKind::Ty(self.lower_to_ty(child, child.child(0).unwrap())?);
            }

            bail!("Cannot lower '{}' to 'SizeofKind'.", node.kind());
        };

        Ok(sizeof_kind)
    }

    pub(crate) fn lower_to_lit(&mut self, node: Node) -> anyhow::Result<Lit> {
        trace!("[HIR/Lit] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(Lit {
            kind: self.lower_to_lit_kind(node).with_context(|| {
                format!(
                    "In lowering {:?} to literal",
                    node.utf8_text(self.source_code)
                )
            })?,
            span,
        })
    }

    fn lower_to_lit_kind(&mut self, node: Node) -> anyhow::Result<LitKind> {
        trace!("[HIR/LitKind] Lowering '{}'", node.kind());

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

                let literal = if let Some(literal) = literal.strip_suffix("U") {
                    literal
                } else if let Some(literal) = literal.strip_suffix("LL") {
                    literal
                } else if let Some(literal) = literal.strip_suffix("L") {
                    literal
                } else {
                    literal
                };

                if let Ok(value) = literal.parse() {
                    LitKind::Int(value)
                } else if let Some(stripped_literal) = literal.strip_prefix("0x")
                    && let Ok(value) = i64::from_str_radix(stripped_literal, 16)
                {
                    LitKind::Int(value)
                } else if let Some(stripped_literal) = literal.strip_prefix("0b")
                    && let Ok(value) = i64::from_str_radix(stripped_literal, 2)
                {
                    LitKind::Int(value)
                } else {
                    LitKind::Float(literal.parse()?)
                }
            }
            kind => bail!("Cannot lower '{kind}' to 'Lit'."),
        })
    }

    // TODO: this function is garbage. Break it into two. One for assignments and one for normal operators.
    fn lower_to_bin_op(&mut self, node: Node) -> anyhow::Result<Option<BinOp>> {
        trace!("[HIR/BinOp] Lowering '{}'", node.kind());

        Ok(Some(match node.kind() {
            constants::ADD | constants::ASSIGN_ADD | constants::INC => BinOp::Add,
            constants::SUB | constants::ASSIGN_SUB | constants::DEC => BinOp::Sub,
            constants::MUL | constants::ASSIGN_MUL => BinOp::Mul,
            constants::DIV | constants::ASSIGN_DIV => BinOp::Div,
            constants::REM | constants::ASSIGN_REM => BinOp::Rem,
            constants::AND => BinOp::And,
            constants::OR => BinOp::Or,
            constants::BIT_XOR | constants::ASSIGN_BIT_XOR => BinOp::BitXor,
            constants::BIT_AND | constants::ASSIGN_BIT_AND => BinOp::BitAnd,
            constants::BIT_OR | constants::ASSIGN_BIT_OR => BinOp::BitOr,
            constants::SHL | constants::ASSIGN_SHL => BinOp::Shl,
            constants::SHR | constants::ASSIGN_SHR => BinOp::Shr,
            constants::EQ => BinOp::Eq,
            constants::LT => BinOp::Lt,
            constants::LE => BinOp::Le,
            constants::NE => BinOp::Ne,
            constants::GE => BinOp::Ge,
            constants::GT => BinOp::Gt,
            constants::ASSIGN => return Ok(None),
            kind => bail!("Cannot lower '{kind}' to 'BinOp'."),
        }))
    }

    fn lower_to_un_op(&mut self, node: Node) -> anyhow::Result<UnOp> {
        trace!("[HIR/UnOp] Lowering '{}'", node.kind());

        Ok(match node.kind() {
            constants::NOT => UnOp::Not,
            constants::NEG => UnOp::Neg,
            constants::COM => UnOp::Com,
            constants::POS => UnOp::Pos,
            constants::ADDR_OF => UnOp::AddrOf,
            constants::DEREF => UnOp::Deref,
            kind => bail!("Cannot lower '{kind}' to 'UnOp'."),
        })
    }
}
