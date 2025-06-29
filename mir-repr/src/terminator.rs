use smallvec::SmallVec;

use hir_repr::Span;

use crate::{basic_block::BasicBlock, operand::Operand};

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
