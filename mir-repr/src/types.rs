use hir_repr::Span;

#[derive(Debug, Clone)]
pub enum Const {
    Val,
}

#[derive(Debug, Clone)]
pub enum PrimTyKind {
    Int,
    Float,
    Double,
    Char,
    Void,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    PrimTy(PrimTyKind),
    Array(Box<Ty>, Const),
    Ptr(Box<Ty>),
}

#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}
