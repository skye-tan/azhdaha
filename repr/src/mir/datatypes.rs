#![allow(clippy::missing_docs_in_private_items)]

use la_arena::{Arena, Idx};
use smallvec::SmallVec;

use crate::hir::{
    BinOp, Lit, Span, Ty, UnOp,
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
    pub debug_name: Option<String>,
    pub ty: Ty,
    pub span: Span,
}

pub type BasicBlock = Idx<BasicBlockData>;

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
        targets: SwitchTargets,
    },
    Return,
}

#[derive(Debug, Clone)]
pub struct SwitchTargets {
    pub value: SmallVec<[u128; 1]>,
    pub bbs: SmallVec<[BasicBlock; 2]>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Assign(Place, Rvalue),
}

#[derive(Debug, Clone)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinOp, Operand, Operand),
    UnaryOp(UnOp, Operand),
    Call(Operand, Vec<Operand>),
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
}

#[derive(Debug, Clone)]
pub enum PlaceElem {
    Deref,
    Index(Local),
}

#[derive(Debug, Clone)]
pub enum Const {
    Lit(Lit),
    Symbol(Symbol),
}
