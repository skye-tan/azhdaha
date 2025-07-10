#![allow(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]

use std::cell::RefCell;

use smallvec::SmallVec;

use hir_repr::Span;

#[derive(Debug, Clone)]
pub enum Const {
    Val,
}

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

#[derive(Debug, Clone)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinOp, Box<(Operand, Operand)>),
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

#[derive(Debug, Clone)]
pub struct BasicBlock {
    index: u32,
}

#[derive(Debug, Clone)]
pub struct BasicBlockData {
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub ty: hir_repr::Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub basic_blocks: Vec<BasicBlockData>,
    pub local_decls: <LocalDecl>,
    pub span: Span,
}

impl Body {
    pub fn print(&self) {
        for (i, lc) in self.local_decls.iter().enumerate() {
            println!("let {}: {:?};", i, lc.ty);
        }
        
        for (i, bb) in self.basic_blocks.iter().enumerate() {
            println!("'bb{i}: {{");
            for stmt in &bb.statements {
                match &stmt.kind {
                    StatementKind::Assign(place, rvalue) => {
                        println!("{:?} = {:?}", place, rvalue);
                    },
                }
            }
            println!("}}");
        }
    }
}

#[derive(Debug, Clone)]
pub struct MirCtx {
    pub input: hir_repr::Fn,
    pub result: RefCell<Body>,
}

