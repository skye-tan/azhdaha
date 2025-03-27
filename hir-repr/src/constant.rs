//! Words used by tree-sitter-c library as constant variables.

/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Add`].
pub(crate) const ADD: &str = "+";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Sub`].
pub(crate) const SUB: &str = "-";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Mul`].
pub(crate) const MUL: &str = "*";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Div`].
pub(crate) const DIV: &str = "/";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Rem`].
pub(crate) const REM: &str = "%";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::And`].
pub(crate) const AND: &str = "&&";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Or`].
pub(crate) const OR: &str = "||";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::BitXor`].
pub(crate) const BIT_XOR: &str = "^";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::BitAnd`].
pub(crate) const BIT_AND: &str = "&";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::BitOr`].
pub(crate) const BIT_OR: &str = "|";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Shl`].
pub(crate) const SHL: &str = "<<";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Shr`].
pub(crate) const SHR: &str = ">>";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Eq`].
pub(crate) const EQ: &str = "==";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Lt`].
pub(crate) const LT: &str = "<";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Le`].
pub(crate) const LE: &str = "<=";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Ne`].
pub(crate) const NE: &str = "!=";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Ge`].
pub(crate) const GE: &str = ">=";
/// Constant word used for [`BinOpKind`]'s construction representing [`BinOpKind::Gt`].
pub(crate) const GT: &str = ">";

/// Constant word used for [`UnOp`]'s construction representing [`UnOp::Not`].
pub(crate) const NOT: &str = "!";
/// Constant word used for [`UnOp`]'s construction representing [`UnOp::Neg`].
pub(crate) const NEG: &str = "-";
/// Constant word used for [`UnOp`]'s construction representing [`UnOp::Com`].
pub(crate) const COM: &str = "~";
/// Constant word used for [`UnOp`]'s construction representing [`UnOp::Pos`].
pub(crate) const POS: &str = "+";

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

/// Constant word used for [`LitKind`]'s construction representing [`LitKind::Str`].
pub(crate) const STRING_LITERAL: &str = "string_literal";
/// Constant word used for [`LitKind`]'s construction representing [`LitKind::Char`].
pub(crate) const CHAR_LITERAL: &str = "char_literal";
/// Constant word used for [`LitKind`]'s construction representing [`LitKind::Int`] or [`LitKind::Float`].
pub(crate) const NUMBER_LITERAL: &str = "number_literal";

/// Constant word used for [`StmtKind`]'s construction representing [`StmtKind::Decl`].
pub(crate) const DECLARATION: &str = "declaration";
/// Constant word used for [`DeclStmt`]'s construction determining whether the initialization part exits or not.
pub(crate) const INIT_DECLARATOR: &str = "init_declarator";

/// Constant word used for [`StmtKind`]'s and [`ExprKind`]'s construction representing [`StmtKind::Expr`] and [`ExprKind::Ret`] respectively.
pub(crate) const RETURN_STATEMENT: &str = "return_statement";
/// Constant word used for [`StmtKind`]'s and [`ExprKind`]'s construction representing [`StmtKind::Expr`] and [`ExprKind::Call`] respectively.
pub(crate) const EXPRESSION_STATEMENT: &str = "expression_statement";
/// Constant word used for [`StmtKind`]'s and [`ExprKind`]'s construction representing [`StmtKind::Expr`] and [`ExprKind::If`] respectively.  
pub(crate) const IF_STATEMENT: &str = "if_statement";
/// Constant word used for [`StmtKind`]'s and [`ExprKind`]'s construction representing [`StmtKind::Expr`] and [`ExprKind::Loop`] respectively.  
pub(crate) const WHILE_STATEMENT: &str = "while_statement";

/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Block`].
pub(crate) const COMPOUND_STATEMENT: &str = "compound_statement";
/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Path`].  
pub(crate) const IDENTIFIER: &str = "identifier";
/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Call`].  
pub(crate) const CALL_EXPRESSION: &str = "call_expression";
/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Binary`].  
pub(crate) const BINARY_EXPRESSION: &str = "binary_expression";
/// Constant word used for [`ExprKind`]'s construction representing a [`ExprKind::Unary`].  
pub(crate) const UNARY_EXPRESSION: &str = "unary_expression";
/// Constant word used for [`ExprKind`]'s construction representing an [`ExprKind`].  
pub(crate) const PARENTHESIZED_EXPRESSION: &str = "parenthesized_expression";
/// Constant word used for [`ExprKind`]'s construction representing an [`ExprKind::Assign`].  
pub(crate) const ASSIGNMENT_EXPRESSION: &str = "assignment_expression";
