use tree_sitter::Tree;

pub struct Span {
    lo: u32,
    len: u16,
}

pub enum PrimTyKind {
    Int,
}

pub enum TyKind {
    PrimTy(PrimTyKind),
}

pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

pub struct DeclStmt<'hir> {
    pub ty: Option<&'hir Ty>,
    pub init: Option<&'hir Expr<'hir>>,
    pub span: Span,
}

pub enum StmtKind<'hir> {
    Decl(&'hir DeclStmt<'hir>),
    Expr(&'hir Expr<'hir>),
}

pub struct Stmt<'hir> {
    pub kind: StmtKind<'hir>,
    pub span: Span,
}

pub struct Block<'hir> {
    pub stmts: &'hir [Stmt<'hir>],
    pub span: Span,
}

pub enum ExprKind<'hir> {
    Block(&'hir Block<'hir>),
}

pub struct Expr<'hir> {
    pub kind: ExprKind<'hir>,
    pub span: Span,
}

impl<'hir> Expr<'hir> {
    pub fn from_ast(tree: &Tree) -> Vec<Self> {
        todo!()
    }
}
