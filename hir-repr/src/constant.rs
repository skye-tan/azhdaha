//! Words used by tree-sitter-c library as constant variables.

/// Constant word used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Int`].
pub(crate) const INT: &str = "int";
/// Constant word used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Float`].
pub(crate) const FLOAT: &str = "float";
/// Constant word used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Double`].
pub(crate) const DOUBLE: &str = "double";
/// Constant word used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Char`].
pub(crate) const CHAR: &str = "char";

/// Constant word used for [`TyKind`]'s construction representing a [`TyKind::PrimTy`].
pub(crate) const PRIMITIVE_TYPE: &str = "primitive_type";

/// Constant word used for [`StmtKind`]'s construction representing [`StmtKind::Decl`].
pub(crate) const DECLARATION: &str = "declaration";
/// Constant word used for [`StmtKind`]'s and [`ExprKind`]'s construction representing [`StmtKind::Expr`] and [`ExprKind::Ret`] respectively.
pub(crate) const RETURN_STATEMENT: &str = "return_statement";

/// Constant word used for [`LitKind`]'s construction representing [`LitKind::Str`].
pub(crate) const STRING_LITERAL: &str = "string_literal";
/// Constant word used for [`LitKind`]'s construction representing [`LitKind::Char`].
pub(crate) const CHAR_LITERAL: &str = "char_literal";
/// Constant word used for [`LitKind`]'s construction representing [`LitKind::Int`] or [`LitKind::Float`].
pub(crate) const NUMBER_LITERAL: &str = "number_literal";

/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Block`].
pub(crate) const COMPOUND_STATEMENT: &str = "compound_statement";
/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Path`].  
pub(crate) const IDENTIFIER: &str = "identifier";
/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Call`].  
pub(crate) const CALL_EXPRESSION: &str = "call_expression";
