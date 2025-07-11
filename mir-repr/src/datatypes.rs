#![allow(clippy::missing_docs_in_private_items)]

use std::{cell::RefCell, collections::HashMap};

use la_arena::{Arena, Idx};
use smallvec::SmallVec;

use hir_repr::{BinOp, Resolver, ResolverIdx, Span, Ty, UnOp};

#[derive(Debug, Clone)]
pub enum Const {
    Val,
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

#[derive(Debug, Clone)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinOp, Box<Operand>, Box<Operand>),
    UnaryOp(UnOp, Operand),
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Assign(Place, Rvalue),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SwitchTargets {
    value: SmallVec<[u128; 1]>,
    targets: SmallVec<[BasicBlock; 2]>,
}

#[derive(Debug, Clone)]
pub enum TerminatorKind {
    Goto {
        target: BasicBlock,
    },
    SwitchInt {
        discr: Operand,
        targets: SwitchTargets,
    },
    Return,
}

#[derive(Debug, Clone)]
pub struct Terminator {
    pub kind: TerminatorKind,
    pub span: Span,
}

pub type BasicBlock = Idx<BasicBlockData>;

#[derive(Debug, Clone, Default)]
pub struct BasicBlockData {
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

pub type Local = Idx<LocalDecl>;

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub basic_blocks: Arena<BasicBlockData>,
    pub local_decls: Arena<LocalDecl>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MirCtx<'mir> {
    pub body: RefCell<Body>,
    pub resolver: &'mir Resolver,
    pub map: HashMap<ResolverIdx, Local>,
}
