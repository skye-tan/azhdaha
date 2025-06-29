use hir_repr::Span;

use crate::types::Const;

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
    Com,
    AddrOf,
    Deref,
}

#[derive(Debug, Clone)]
pub struct Local {
    index: u32,
}

#[derive(Debug, Clone)]
pub enum PlaceElem {
    Deref,
    Index(Local),
}

#[derive(Debug, Clone)]
pub struct Place {
    pub local: Local,
    pub projections: Vec<PlaceElem>,
}

#[derive(Debug, Clone)]
pub struct ConstOperand {
    pub cons_: Const,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Place(Place),
    Constant(ConstOperand),
}
