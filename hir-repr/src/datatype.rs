#[derive(Debug)]
pub struct Span {
    pub lo: u32,
    pub len: u16,
}

#[derive(Debug)]
pub enum PrimTyKind {
    Int,
    Float,
    Double,
    Char,
}

#[derive(Debug)]
pub enum TyKind {
    PrimTy(PrimTyKind),
}

#[derive(Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct DeclStmt {
    pub ty: Ty,
    pub ident: Ident,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    Decl(DeclStmt),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug)]
pub enum LitIntType {
    Signed,
    Unsigned,
}

#[derive(Debug)]
pub enum LitKind {
    Int(i128, LitIntType),
}

#[derive(Debug)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Block(Block),
    Lit(Lit),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
