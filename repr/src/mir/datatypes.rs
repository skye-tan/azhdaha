#![allow(clippy::missing_docs_in_private_items)]

use la_arena::{Arena, Idx};

use crate::hir::{
    BinOp, Ident, Lit, Span, Storage, Ty, TyKind, UnOp,
    resolver::{CompoundTypeData, Resolver, Symbol, SymbolKind},
};

#[derive(Debug, Clone)]
pub struct Body<'mir> {
    pub symbol_resolver: &'mir Resolver<SymbolKind>,

    pub local_decls: Arena<LocalDecl>,
    pub basic_blocks: Arena<BasicBlockData>,

    pub span: Span,
}

pub type Local = Idx<LocalDecl>;

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub ty: Ty,
    pub kind: LocalKind,
    pub span: Span,
}

impl LocalDecl {
    pub fn is_linear(&self) -> bool {
        self.ty.is_linear
    }
}

#[derive(Debug, Clone)]
pub enum LocalKind {
    Real {
        storage: Option<Storage>,
        ident: Ident,
        is_arg: bool,
    },
    Temp,
}

#[derive(Debug, Clone, Copy)]
pub struct BasicBlock(Idx<BasicBlockData>);

impl BasicBlock {
    pub fn set(&mut self, bb: BasicBlock) {
        self.0 = bb.0;
    }

    pub fn get_id(&self) -> usize {
        self.0.into_raw().into_u32() as usize
    }

    pub fn into_inner(self) -> Idx<BasicBlockData> {
        self.0
    }
}

impl From<Idx<BasicBlockData>> for BasicBlock {
    fn from(value: Idx<BasicBlockData>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BasicBlockData {
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug, Clone)]
pub struct Terminator {
    pub kind: TerminatorKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TerminatorKind {
    Goto {
        bb: BasicBlock,
    },
    SwitchInt {
        discr: Operand,
        targets: [BasicBlock; 2],
    },
    Return,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Assign(Place, Rvalue),
    Call(Operand, Vec<Operand>),
}

#[derive(Debug, Clone, Copy)]
pub enum IntBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
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
pub enum ShortCircuitBinOp {
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum MirBinOp {
    IntBinOp(IntBinOp),
    ShortCircuitBinOp(ShortCircuitBinOp),
}

impl MirBinOp {
    pub fn from_hir(binop: BinOp) -> MirBinOp {
        match binop {
            BinOp::Add => MirBinOp::IntBinOp(IntBinOp::Add),
            BinOp::Sub => MirBinOp::IntBinOp(IntBinOp::Sub),
            BinOp::Mul => MirBinOp::IntBinOp(IntBinOp::Mul),
            BinOp::Div => MirBinOp::IntBinOp(IntBinOp::Div),
            BinOp::Rem => MirBinOp::IntBinOp(IntBinOp::Rem),
            BinOp::Or => MirBinOp::ShortCircuitBinOp(ShortCircuitBinOp::Or),
            BinOp::And => MirBinOp::ShortCircuitBinOp(ShortCircuitBinOp::And),
            BinOp::BitOr => MirBinOp::IntBinOp(IntBinOp::BitOr),
            BinOp::BitXor => MirBinOp::IntBinOp(IntBinOp::BitXor),
            BinOp::BitAnd => MirBinOp::IntBinOp(IntBinOp::BitAnd),
            BinOp::Eq => MirBinOp::IntBinOp(IntBinOp::Eq),
            BinOp::Lt => MirBinOp::IntBinOp(IntBinOp::Lt),
            BinOp::Le => MirBinOp::IntBinOp(IntBinOp::Le),
            BinOp::Ne => MirBinOp::IntBinOp(IntBinOp::Ne),
            BinOp::Ge => MirBinOp::IntBinOp(IntBinOp::Ge),
            BinOp::Gt => MirBinOp::IntBinOp(IntBinOp::Gt),
            BinOp::Shl => MirBinOp::IntBinOp(IntBinOp::Shl),
            BinOp::Shr => MirBinOp::IntBinOp(IntBinOp::Shr),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IntUnOp {
    Not,
    Neg,
    Com,
    Pos,
}

#[derive(Debug, Clone, Copy)]
pub enum MirUnOp {
    IntUnOp(IntUnOp),
    AddrOf,
    Deref,
}

impl MirUnOp {
    pub fn from_hir(un_op: UnOp) -> MirUnOp {
        match un_op {
            UnOp::Not => MirUnOp::IntUnOp(IntUnOp::Not),
            UnOp::Neg => MirUnOp::IntUnOp(IntUnOp::Neg),
            UnOp::Com => MirUnOp::IntUnOp(IntUnOp::Com),
            UnOp::Pos => MirUnOp::IntUnOp(IntUnOp::Pos),
            UnOp::AddrOf => MirUnOp::AddrOf,
            UnOp::Deref => MirUnOp::Deref,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(IntBinOp, Operand, Operand),
    PtrDiff(Operand, Operand),
    UnaryOp(IntUnOp, Operand),
    AddrOf(Place),
    AddrOfStatic(Symbol),
    Call(Operand, Vec<Operand>),
    Cast {
        value: Operand,
        from_type: TyKind,
        to_type: TyKind,
    },
    StructInitializing(Idx<CompoundTypeData>, Vec<Operand>),
    List(Vec<Operand>),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Place(Place),
    Const(Const),
}

#[derive(Debug, Clone)]
pub struct Place {
    pub local: Local,
    pub projections: Vec<PlaceElem>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum PlaceElem {
    Field(String),
    Index(Place),
    Deref,
}

#[derive(Debug, Clone)]
pub enum Const {
    Lit(Lit),
    Symbol(Symbol),
    Sizeof(Ty),
}
