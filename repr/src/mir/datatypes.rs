#![allow(clippy::missing_docs_in_private_items)]

use la_arena::{Arena, Idx};

use crate::hir::{
    BinOp, Ident, Lit, Span, Storage, Ty, UnOp,
    resolver::{Resolver, Symbol, SymbolKind},
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
    pub kind: LocalKind,
    pub span: Span,
}

impl LocalDecl {
    pub fn is_linear(&self) -> bool {
        match &self.kind {
            LocalKind::Real { ty, .. } => ty.is_linear,
            LocalKind::Temp => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalKind {
    Real {
        storage: Option<Storage>,
        ty: Ty,
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

#[derive(Debug, Clone)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinOp, Operand, Operand),
    UnaryOp(UnOp, Operand),
    Call(Operand, Vec<Operand>),
    Cast(Operand, Ty),
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
    Sizeof,
}
