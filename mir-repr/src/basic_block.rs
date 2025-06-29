use hir_repr::Span;

use crate::{statement::Statement, terminator::Terminator, types::Ty};

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
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub basic_blocks: Vec<BasicBlockData>,
    pub local_decls: Vec<LocalDecl>,
    pub span: Span,
}
