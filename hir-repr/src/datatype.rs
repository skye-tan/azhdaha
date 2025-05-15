#![allow(clippy::missing_docs_in_private_items)]

use std::collections::HashMap;

use la_arena::{Arena, Idx};
use tree_sitter::TreeCursor;

#[derive(Debug, Clone)]
pub struct ResCtx<T> {
    map: HashMap<String, Idx<T>>,
    arena: Arena<T>,
}

impl<T> ResCtx<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            arena: Arena::new(),
        }
    }

    pub fn insert(&mut self, name: String, elem: T) {
        let idx = self.arena.alloc(elem);
        self.map.insert(name, idx);
    }

    pub fn retrieve(&self, name: &str) -> Option<&T> {
        self.map.get(name).map(|idx| &self.arena[*idx])
    }
}

impl<T> Default for ResCtx<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
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
    pub ty: Ty,
    pub ident: Ident,
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
    pub res_ctx: ResCtx<DeclStmt>,
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

#[derive(Debug, Clone)]
pub struct Path {
    pub res: Ident, // TODO: use Res instead of Ident
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum BinOpKind {
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

#[derive(Debug, Clone)]
pub struct BinOp {
    pub node: BinOpKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
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
    Path(Path),
    Call(Box<Expr>, Vec<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Loop(LoopSource, Box<Expr>),
    Break,
    Continue,
    Assign(Box<Expr>, Box<Expr>),
    AssignOp(BinOp, Box<Expr>, Box<Expr>),
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
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub struct FnSig {
    pub ty: Ty,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub sig: FnSig,
    pub body: Expr,
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

pub struct LoweringCtx<'a> {
    pub items: Vec<Item>,
    pub res_ctx: ResCtx<FnSig>,
    pub cursor: TreeCursor<'a>,
    pub source_code: &'a [u8],
}
