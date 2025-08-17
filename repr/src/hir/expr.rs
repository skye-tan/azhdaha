#![allow(clippy::missing_docs_in_private_items)]

use anyhow::{Context, bail};
use log::trace;

use crate::hir::*;

use super::{constants, resolver::Symbol};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
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
    Index(Box<Expr>, Box<Expr>),
    Cast(Box<Expr>, Ty),
    Array(Vec<Expr>),
    Comma(Vec<Expr>),
    Sizeof(Sizeof),
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
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

#[derive(Debug, Clone, Copy)]
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
    Assign,
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

        Ok(Expr {
            kind: self.lower_to_expr_kind(node)?,
            span,
        })
    }

    fn lower_to_expr_kind(&mut self, node: Node) -> anyhow::Result<ExprKind> {
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

                ExprKind::Local(symbol)
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

                ExprKind::Call(Box::new(path), arguments)
            }
            constants::BINARY_EXPRESSION => {
                let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                let bin_op = self.lower_to_bin_op(node.child(1).unwrap())?;

                let rhs = self.lower_to_expr(node.child(2).unwrap())?;

                ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs))
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
                };

                ExprKind::Assign(
                    Box::new(lhs.clone()),
                    Box::new(Expr {
                        kind: ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs)),
                        span,
                    }),
                )
            }
            constants::UNARY_EXPRESSION | constants::POINTER_EXPRESSION => {
                let un_op = self.lower_to_un_op(node.child(0).unwrap())?;

                let expr = self.lower_to_expr(node.child(1).unwrap())?;

                // Ignore [`UnOp::Pos`] because it has no effects.
                match un_op {
                    UnOp::Pos => expr.kind,
                    _ => ExprKind::Unary(un_op, Box::new(expr)),
                }
            }
            constants::PARENTHESIZED_EXPRESSION => {
                self.lower_to_expr_kind(node.child(1).unwrap())?
            }
            constants::ASSIGNMENT_EXPRESSION => {
                let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                let bin_op = self.lower_to_bin_op(node.child(1).unwrap())?;

                let rhs = self.lower_to_expr(node.child(2).unwrap())?;

                match bin_op {
                    BinOp::Assign => ExprKind::Assign(Box::new(lhs), Box::new(rhs)),
                    _ => ExprKind::Assign(
                        Box::new(lhs.clone()),
                        Box::new(Expr {
                            kind: ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs)),
                            span,
                        }),
                    ),
                }
            }
            constants::FIELD_EXPRESSION => {
                let target = self.lower_to_expr(node.child(0).unwrap())?;

                let field = self.lower_to_ident(node.child(2).unwrap())?;

                ExprKind::Field(Box::new(target), field)
            }
            constants::SUBSCRIPT_EXPRESSION => {
                let target = self.lower_to_expr(node.child(0).unwrap())?;

                let index = self.lower_to_expr(node.child(2).unwrap())?;

                ExprKind::Index(Box::new(target), Box::new(index))
            }
            constants::CAST_EXPRESSION => {
                let cast_node = node.child(1).unwrap();

                let ty = self.lower_to_ty(
                    cast_node,
                    cast_node.child_by_field_name("declarator").unwrap(),
                )?;

                let target = self.lower_to_expr(node.child(3).unwrap())?;

                ExprKind::Cast(Box::new(target), ty)
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

                ExprKind::Array(elements)
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

                ExprKind::Comma(exprs)
            }
            constants::CONDITIONAL_EXPRESSION => {
                let cond_expr = self.lower_to_expr(node.child(0).unwrap())?;

                let body_expr = self.lower_to_expr(node.child(2).unwrap())?;

                let else_expr = self.lower_to_expr(node.child(4).unwrap())?;

                ExprKind::Cond(
                    Box::new(cond_expr),
                    Box::new(body_expr),
                    Box::new(else_expr),
                )
            }
            constants::SIZEOF_EXPRESSION => ExprKind::Sizeof(self.lower_to_sizeof(node)?),
            constants::SEMICOLON_EXPRESSION => ExprKind::Empty,
            kind if kind.contains(constants::LITERAL) => ExprKind::Lit(self.lower_to_lit(node)?),
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
                break 'size_of SizeofKind::Ty(
                    self.lower_to_ty(child, child.child_by_field_name("declarator").unwrap())?,
                );
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
            kind: self.lower_to_lit_kind(node)?,
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

                if let Ok(value) = literal.parse() {
                    LitKind::Int(value)
                } else {
                    LitKind::Float(literal.parse()?)
                }
            }
            kind => bail!("Cannot lower '{kind}' to 'Lit'."),
        })
    }

    fn lower_to_bin_op(&mut self, node: Node) -> anyhow::Result<BinOp> {
        trace!("[HIR/BinOp] Lowering '{}'", node.kind());

        Ok(match node.kind() {
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
            constants::ASSIGN => BinOp::Assign,
            kind => bail!("Cannot lower '{kind}' to 'BinOp'."),
        })
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
