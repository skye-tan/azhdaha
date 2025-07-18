#![allow(clippy::missing_docs_in_private_items)]

use tree_sitter::TreeCursor;

use crate::hir::resolver::{Label, Resolver, Symbol, SymbolKind};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug, Clone)]
pub enum PrimTyKind {
    Int,
    Bool,
    Float,
    Double,
    Char,
    Void,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    PrimTy(PrimTyKind),
    Array(Box<TyKind>, Box<Expr>),
    Ptr(Box<TyKind>),
}

#[derive(Debug, Clone)]
pub enum TyQual {
    Const,
    Volatile,
    Atomic,
    Linear,
}

#[derive(Debug, Clone)]
pub struct Ty {
    pub quals: Vec<TyQual>,
    pub kind: TyKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub symbol_resolver: Resolver<SymbolKind>,

    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub ty: Ty,
    pub ident: Ident,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclStmt {
    pub decls: Vec<Symbol>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Block(Block),
    Expr(Expr),
    Decl(DeclStmt),
    Ret(Option<Expr>),
    Label(Label, Option<Box<Stmt>>),
    Goto(Label),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
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
    Lit(Lit),
    Local(Symbol),
    Call(Box<Expr>, Vec<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Field(Box<Expr>, Ident),
    Index(Box<Expr>, Box<Expr>, Span),
    Cast(Box<Expr>, Ty),
    Array(Vec<Expr>),
    AddrOf(Box<Expr>),
    Comma(Vec<Expr>),
    Sizeof(Sizeof),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Ty,
    pub ident: Option<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FuncSig {
    pub ret_ty: Ty,
    pub ident: Ident,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub label_resolver: Resolver<()>,

    pub sig: Symbol,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Func(Func),
    GlobalVar(DeclStmt),
    ProtoType(Symbol),
    Struct,
    Union,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

pub struct LoweringCtx<'hir> {
    pub symbol_resolver: Resolver<SymbolKind>,
    pub label_resolver: Resolver<()>,

    pub items: Vec<Item>,

    pub cursor: TreeCursor<'hir>,
    pub source_code: &'hir [u8],
}
