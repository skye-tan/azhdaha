#[derive(Debug)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
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
pub enum LitKind {
    Str(String),
    Char(char),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct Path {
    pub res: Ident, // TODO: use Res instead of Ident
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Block(Block),
    Lit(Lit),
    Ret(Box<Expr>),
    Path(Path),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
