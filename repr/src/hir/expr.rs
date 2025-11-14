#![allow(clippy::missing_docs_in_private_items)]

use anyhow::{Context, bail};
use itertools::Either;
use log::trace;

use crate::hir::{
    resolver::{CompoundTypeData, SymbolKind},
    *,
};

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
    Int(i128),
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

impl BinOp {
    const COMPARISONS: &[Self] = &[
        BinOp::Eq,
        BinOp::Le,
        BinOp::Ge,
        BinOp::Gt,
        BinOp::Lt,
        BinOp::Ne,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
    Com,
    Pos,
    AddrOf,
    Deref,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_expr_with_expected_type(
        &mut self,
        node: Node,
        ty: Ty,
    ) -> anyhow::Result<Expr> {
        let expr = self.lower_to_expr(node)?;

        Ok(Expr {
            span: expr.span,
            kind: ExprKind::Cast(Box::new(expr)),
            ty,
        })
    }

    pub(crate) fn lower_to_expr(&mut self, node: Node) -> anyhow::Result<Expr> {
        trace!("[HIR/Expr] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let (kind, ty) = self.lower_to_expr_kind(node)?;

        Ok(Expr { kind, ty, span })
    }

    /// This function is for sizeof when it didn't detect type at parse level, e.g. for typedefs.
    pub(crate) fn lower_to_expr_or_type(&mut self, node: Node) -> anyhow::Result<Either<Expr, Ty>> {
        Ok(match node.kind() {
            constants::IDENTIFIER => {
                let ident = self.lower_to_ident(node)?;

                let symbol = self
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .context(format!("Use of undefined identifier '{}'.", &ident.name))?;

                if let SymbolKind::TyDef(ty) = &self.symbol_resolver.arena[symbol] {
                    Either::Right(ty.clone())
                } else {
                    Either::Left(self.lower_to_expr(node)?)
                }
            }
            constants::PARENTHESIZED_EXPRESSION => {
                let child = node.child(1).unwrap();
                self.lower_to_expr_or_type(child)?
            }
            _ => Either::Left(self.lower_to_expr(node)?),
        })
    }

    fn lower_un_op(
        &mut self,
        mut expr: Expr,
        un_op: UnOp,
        span: Span,
    ) -> anyhow::Result<(ExprKind, Ty)> {
        let ty = match un_op {
            UnOp::Not | UnOp::Neg | UnOp::Com | UnOp::Pos => expr.ty.clone(),
            UnOp::AddrOf => {
                if expr.ty.kind.is_array() {
                    let it = self.array_to_pointer_decay(expr);
                    return Ok((it.kind, it.ty));
                }
                Ty {
                    kind: TyKind::Ptr {
                        kind: Box::new(expr.ty.kind.clone()),
                        quals: vec![],
                    },
                    is_linear: false,
                    quals: vec![],
                    span,
                }
            }
            UnOp::Deref => {
                self.array_to_pointer_decay_if_array(&mut expr);
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

        if BinOp::COMPARISONS.contains(&bin_op) {
            self.pointer_to_address_decay_if_pointer(&mut lhs);
            self.pointer_to_address_decay_if_pointer(&mut rhs);
        }

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

    fn pointer_to_address_decay_if_pointer(&mut self, expr: &mut Expr) {
        if !expr.ty.kind.is_ptr() {
            return;
        }

        let ty = Ty {
            kind: TyKind::PrimTy(PrimTyKind::Int),
            is_linear: false,
            quals: vec![],
            span: expr.span,
        };

        *expr = Expr {
            span: expr.span,
            kind: ExprKind::Cast(Box::new(expr.clone())),
            ty,
        };
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

                let sig = match &path.ty.kind {
                    TyKind::Func { sig } => sig,
                    TyKind::Ptr { kind, quals: _ } => match &**kind {
                        TyKind::Func { sig } => sig,
                        _ => bail!("Type error: invalid call to pointer of non function type."),
                    },
                    _ => bail!("Type error: invalid call to non function type."),
                };

                let mut arguments = vec![];

                cursor.goto_next_sibling();
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                while cursor.node().kind() != ")" {
                    if let Some(param) = sig.params.get(arguments.len()) {
                        arguments.push(
                            self.lower_to_expr_with_expected_type(cursor.node(), param.ty.clone())?,
                        );
                    } else if sig.variadic_param {
                        arguments.push(self.lower_to_expr(cursor.node())?);
                    } else {
                        bail!("Type error - too many arguments to call {sig:?}");
                    }
                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();
                }

                let ty = sig.ret_ty.clone();

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

                let ty = lhs.ty.clone();

                (
                    match bin_op {
                        None => {
                            let rhs = self.lower_to_expr_with_expected_type(
                                node.child(2).unwrap(),
                                lhs.ty.clone(),
                            )?;
                            ExprKind::Assign(Box::new(lhs), Box::new(rhs))
                        }
                        Some(bin_op) => {
                            let rhs = self.lower_to_expr(node.child(2).unwrap())?;
                            let (kind, ty) = self.lower_bin_op(lhs.clone(), rhs, bin_op, span)?;
                            ExprKind::Assign(Box::new(lhs), Box::new(Expr { kind, span, ty }))
                        }
                    },
                    ty,
                )
            }
            constants::FIELD_EXPRESSION => {
                let mut target =
                    self.lower_to_expr(node.child_by_field_name("argument").unwrap())?;

                let field = self.lower_to_ident(node.child_by_field_name("field").unwrap())?;

                if node.child(1).unwrap().kind() == "->" {
                    let (kind, ty) = self.lower_un_op(target, UnOp::Deref, span)?;
                    target = Expr { kind, ty, span };
                }

                let fields = match target.ty.kind {
                    TyKind::Struct(idx) => {
                        let data = self.type_tag_resolver.get_data_by_res(&idx);
                        let CompoundTypeData::Struct { fields } = data else {
                            bail!("Invalid struct {data:?}");
                        };
                        fields
                    }
                    TyKind::Union(idx) => {
                        let data = self.type_tag_resolver.get_data_by_res(&idx);
                        let CompoundTypeData::Union { fields } = data else {
                            bail!("Invalid union {data:?}");
                        };
                        fields
                    }
                    _ => bail!(
                        "Type error: field expression on type {} is invalid.",
                        target.ty
                    ),
                };

                let Some(field_data) = fields.iter().find(|f| f.ident.name == field.name) else {
                    bail!(
                        "Unresolved field {}. Available fields are {:?}.",
                        field.name,
                        fields
                    );
                };
                let ty = field_data.ty.clone();
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

                let decl_node = cast_node.child_by_field_name("declarator");

                let ty_kind = self.lower_to_ty_kind(cast_node, decl_node)?;

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
            constants::SEMICOLON => (
                ExprKind::Empty,
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Void),
                    is_linear: false,
                    quals: vec![],
                    span,
                },
            ),
            kind if kind.contains(constants::LITERAL) => {
                let lit = self.lower_to_lit(node)?;
                let kind = match lit.kind {
                    LitKind::Str(_) => TyKind::Ptr {
                        kind: Box::new(TyKind::PrimTy(PrimTyKind::Char)),
                        quals: vec![],
                    },
                    LitKind::Char(_) => TyKind::PrimTy(PrimTyKind::Char),
                    LitKind::Int(_) => TyKind::PrimTy(PrimTyKind::Int),
                    LitKind::Float(_) => TyKind::PrimTy(PrimTyKind::Float),
                };
                (
                    ExprKind::Lit(lit),
                    Ty {
                        kind,
                        is_linear: false,
                        quals: vec![],
                        span,
                    },
                )
            }
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
            if let Some(child) = node.child_by_field_name("value") {
                break 'size_of match self.lower_to_expr_or_type(child)? {
                    Either::Left(expr) => SizeofKind::Expr(Box::new(expr)),
                    Either::Right(ty) => SizeofKind::Ty(ty),
                };
            }

            if let Some(child) = node.child_by_field_name("type") {
                break 'size_of SizeofKind::Ty(
                    self.lower_to_ty(child, child.child_by_field_name("declarator"))?,
                );
            }

            bail!("Cannot lower '{}' to 'SizeofKind'.", node.to_sexp());
        };

        Ok(sizeof_kind)
    }

    pub(crate) fn lower_to_lit(&self, node: Node) -> anyhow::Result<Lit> {
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

    fn lower_to_lit_kind(&self, node: Node) -> anyhow::Result<LitKind> {
        trace!("[HIR/LitKind] Lowering '{}'", node.kind());

        Ok(match node.kind() {
            constants::STRING_LITERAL => {
                let text = &self.source_code[node.start_byte() + 1..node.end_byte() - 1];
                let text = std::str::from_utf8(text)?;
                LitKind::Str(unescaper::unescape(text)?)
            }
            constants::CHAR_LITERAL => {
                let text = &self.source_code[node.start_byte() + 1..node.end_byte() - 1];
                let text = std::str::from_utf8(text)?;
                let text = unescaper::unescape(text)?;
                LitKind::Char(text.as_bytes()[0] as char)
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

                if let Some(stripped_literal) = literal.strip_prefix("0x") {
                    LitKind::Int(i128::from_str_radix(stripped_literal, 16)?)
                } else if let Some(stripped_literal) = literal.strip_prefix("0X") {
                    LitKind::Int(i128::from_str_radix(stripped_literal, 16)?)
                } else if let Some(stripped_literal) = literal.strip_prefix("0b") {
                    LitKind::Int(i128::from_str_radix(stripped_literal, 2)?)
                } else if let Some(stripped_literal) = literal.strip_prefix("0B") {
                    LitKind::Int(i128::from_str_radix(stripped_literal, 2)?)
                } else if let Some(stripped_literal) = literal.strip_prefix("0") {
                    if stripped_literal.is_empty() {
                        LitKind::Int(0)
                    } else {
                        LitKind::Int(i128::from_str_radix(stripped_literal, 8)?)
                    }
                } else if let Ok(value) = literal.parse() {
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
