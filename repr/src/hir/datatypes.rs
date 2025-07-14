#![allow(clippy::missing_docs_in_private_items)]

use tree_sitter::TreeCursor;

use crate::hir::resolver::{ResIdx, Resolver};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
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
    Array(Box<Ty>, Box<Expr>),
    Ptr(Box<Ty>),
}

#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclStmt {
    pub res: ResIdx,
    pub ty: Ty,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Decl(DeclStmt),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub resolver: Resolver,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LitKind {
    Str(String),
    Char(char),
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
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
    Assign,
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Not,
    Neg,
    Com,
    Pos,
    AddrOf,
    Deref,
}

#[derive(Debug, Clone)]
pub enum LoopSource {
    While,
    DoWhile,
    For,
}

#[derive(Debug, Clone)]
pub enum SizeofKind {
    Ty(Ty),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Sizeof {
    pub kind: SizeofKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Block(Block),
    Lit(Lit),
    Ret(Box<Expr>),
    Local(ResIdx),
    Call(Box<Expr>, Vec<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Loop(LoopSource, Box<Expr>),
    Break,
    Continue,
    Assign(Box<Expr>, Box<Expr>),
    Field(Box<Expr>, Ident),
    Index(Box<Expr>, Box<Expr>, Span),
    Cast(Box<Expr>, Ty),
    Array(Vec<Expr>),
    AddrOf(Box<Expr>),
    Comma(Vec<Expr>),
    Sizeof(Sizeof),
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub res: Option<ResIdx>,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnSig {
    pub res: ResIdx,
    pub ty: Ty,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub sig: FnSig,
    pub body: Expr,
    pub resolver: Resolver,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Fn(Fn),
    Union,
    Struct,
    GlobalVar,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

pub struct LoweringCtx<'hir> {
    pub items: Vec<Item>,

    pub resolver: Resolver,

    pub cursor: TreeCursor<'hir>,
    pub source_code: &'hir [u8],
}
