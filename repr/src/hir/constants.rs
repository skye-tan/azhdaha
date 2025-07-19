//! Identifiers used by tree-sitter-c library to represent nodes.
//!

/// An identifiers used for [`ItemKind`]'s construction representing an [`ItemKind::Func`].  
pub(crate) const FUNCTION_DEFINITION: &str = "function_definition";

/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Block`].
pub(crate) const COMPOUND_STATEMENT: &str = "compound_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Expr`].
pub(crate) const EXPRESSION_STATEMENT: &str = "expression_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Ret`].
pub(crate) const RETURN_STATEMENT: &str = "return_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Label`].
pub(crate) const LABELED_STATEMENT: &str = "labeled_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Goto`].
pub(crate) const GOTO_STATEMENT: &str = "goto_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::If`].
pub(crate) const IF_STATEMENT: &str = "if_statement";
/// An identifiers used for [`StmtKind`]'s construction which will be represented by [`StmtKind::Block`],
/// [`StmtKind::If`], [`StmtKind::Label`], and [`StmtKind::Goto`].
pub(crate) const WHILE_STATEMENT: &str = "while_statement";
/// An identifiers used for [`StmtKind`]'s construction which will be represented by [`StmtKind::Block`],
/// [`StmtKind::If`], [`StmtKind::Label`], and [`StmtKind::Goto`].
pub(crate) const DO_STATEMENT: &str = "do_statement";
/// An identifiers used for [`StmtKind`]'s construction which will be represented by [`StmtKind::Block`],
/// [`StmtKind::If`], [`StmtKind::Label`], and [`StmtKind::Goto`].
pub(crate) const FOR_STATEMENT: &str = "for_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Goto`] .
pub(crate) const BREAK_STATEMENT: &str = "break_statement";
/// An identifiers used for [`StmtKind`]'s construction representing a [`StmtKind::Goto`].
pub(crate) const CONTINUE_STATEMENT: &str = "continue_statement";

/// An identifiers representing a declaration.
pub(crate) const DECLARATION: &str = "declaration";
/// An identifiers representing a declaration with initialization part.
pub(crate) const INIT_DECLARATOR: &str = "init_declarator";
/// An identifiers representing a declaration which is an array.
pub(crate) const ARRAY_DECLARATOR: &str = "array_declarator";
/// An identifiers representing a declaration which is a pointer.
pub(crate) const POINTER_DECLARATOR: &str = "pointer_declarator";
/// An identifiers representing a declaration which is a function.
pub(crate) const FUNCTION_DECLARATOR: &str = "function_declarator";
/// An identifiers determining whether a function has variadic parameter or not.
pub(crate) const VARIADIC_PARAMETER: &str = "variadic_parameter";
/// An identifiers representing a declaration which is a parameter.
pub(crate) const PARAMETER_DECLARATION: &str = "parameter_declaration";
/// An identifiers representing a declaration which is a inside parenthesizes.
pub(crate) const PARENTHESIZED_DECLARATOR: &str = "parenthesized_declarator";

/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Call`].  
pub(crate) const CALL_EXPRESSION: &str = "call_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Binary`].  
pub(crate) const BINARY_EXPRESSION: &str = "binary_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Binary`].  
pub(crate) const UPDATE_EXPRESSION: &str = "update_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Unary`].  
pub(crate) const UNARY_EXPRESSION: &str = "unary_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Unary`].  
pub(crate) const POINTER_EXPRESSION: &str = "pointer_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind`].  
pub(crate) const PARENTHESIZED_EXPRESSION: &str = "parenthesized_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Assign`].  
pub(crate) const ASSIGNMENT_EXPRESSION: &str = "assignment_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Field`].  
pub(crate) const FIELD_EXPRESSION: &str = "field_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Index`].  
pub(crate) const SUBSCRIPT_EXPRESSION: &str = "subscript_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Cast`].  
pub(crate) const CAST_EXPRESSION: &str = "cast_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Array`].  
pub(crate) const INITIALIZER_LIST: &str = "initializer_list";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Comma`].  
pub(crate) const COMMA_EXPRESSION: &str = "comma_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::SizeOf`].  
pub(crate) const SIZEOF_EXPRESSION: &str = "sizeof_expression";
/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Empty`].  
pub(crate) const SEMICOLON_EXPRESSION: &str = ";";

/// An identifiers used for [`Ty`]'s construction determining whether the type is linear or not.
pub(crate) const LINEAR_TY_SPECIFIER: &str = "linear_type_specifier";

/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual`].
pub(crate) const TYPE_QUALIFIER: &str = "type_qualifier";
/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual::Const`].
pub(crate) const CONST: &str = "const";
/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual::Constexpr`].
pub(crate) const CONSTEXPR: &str = "constexpr";
/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual::Volatile`].
pub(crate) const VOLATILE: &str = "volatile";
/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual::Restrict`].
pub(crate) const RESTRICT: &str = "restrict";
/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual::Atomic`].
pub(crate) const ATOMIC: &str = "_Atomic";
/// An identifiers used for [`TyQual`]'s construction representing a [`TyQual::NoReturn`].
pub(crate) const NORETURN: &str = "noreturn";

/// An identifiers used for [`Storage`]'s construction.
pub(crate) const STORAGE_CLASS_SPECIFIER: &str = "storage_class_specifier";
/// An identifiers used for [`Storage`]'s construction representing a [`Storage::Extern`].
pub(crate) const EXTERN: &str = "extern";
/// An identifiers used for [`Storage`]'s construction representing a [`Storage::Static`].
pub(crate) const STATIC: &str = "static";
/// An identifiers used for [`Storage`]'s construction representing a [`Storage::Auto`].
pub(crate) const AUTO: &str = "auto";
/// An identifiers used for [`Storage`]'s construction representing a [`Storage::Register`].
pub(crate) const REGISTER: &str = "register";
/// An identifiers used for [`Storage`]'s construction representing a [`Storage::Inline`].
pub(crate) const INLINE: &str = "inline";
/// An identifiers used for [`Storage`]'s construction representing a [`Storage::ThreadLocal`].
pub(crate) const THREAD_LOCAL: &str = "thread_local";

/// An identifiers used for [`ExprKind`]'s construction representing an [`ExprKind::Local`].  
pub(crate) const IDENTIFIER: &str = "identifier";

/// An identifiers used for [`TyKind`]'s construction representing a [`TyKind::PrimTy`].
pub(crate) const PRIMITIVE_TYPE: &str = "primitive_type";
/// An identifiers used for [`TyKind`]'s construction representing a [`TyKind`].
pub(crate) const TYPE_DESCRIPTOR: &str = "type_descriptor";
/// An identifiers used for [`TyKind`]'s construction..
pub(crate) const TYPE_IDENTIFIER: &str = "type_identifier";

/// An identifiers used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Int`].
pub(crate) const INT: &str = "int";
/// An identifiers used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::BOOL`].
pub(crate) const BOOL: &str = "_Bool";
/// An identifiers used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Float`].
pub(crate) const FLOAT: &str = "float";
/// An identifiers used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Double`].
pub(crate) const DOUBLE: &str = "double";
/// An identifiers used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Char`].
pub(crate) const CHAR: &str = "char";
/// An identifiers used for [`PrimTyKind`]'s construction representing a [`PrimTyKind::Void`].
pub(crate) const VOID: &str = "void";

/// An identifiers used for [`LitKind`]'s construction representing a [`LitKind::Str`].
pub(crate) const STRING_LITERAL: &str = "string_literal";
/// An identifiers used for [`LitKind`]'s construction representing a [`LitKind::Char`].
pub(crate) const CHAR_LITERAL: &str = "char_literal";
/// An identifiers used for [`LitKind`]'s construction representing a [`LitKind::Int`] or [`LitKind::Float`].
pub(crate) const NUMBER_LITERAL: &str = "number_literal";

/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Assign`].
pub(crate) const ASSIGN: &str = "=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Add`].
pub(crate) const ASSIGN_ADD: &str = "+=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Sub`].
pub(crate) const ASSIGN_SUB: &str = "-=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Mul`].
pub(crate) const ASSIGN_MUL: &str = "*=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Div`].
pub(crate) const ASSIGN_DIV: &str = "/=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Rem`].
pub(crate) const ASSIGN_REM: &str = "%=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::BitXor`].
pub(crate) const ASSIGN_BIT_XOR: &str = "^=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::BitAnd`].
pub(crate) const ASSIGN_BIT_AND: &str = "&=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::BitOr`].
pub(crate) const ASSIGN_BIT_OR: &str = "|=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Shl`].
pub(crate) const ASSIGN_SHL: &str = "<<=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Eq`].
pub(crate) const ASSIGN_SHR: &str = ">>=";

/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Add`].
pub(crate) const INC: &str = "++";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Sub`].
pub(crate) const DEC: &str = "--";

/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Add`].
pub(crate) const ADD: &str = "+";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Sub`].
pub(crate) const SUB: &str = "-";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Mul`].
pub(crate) const MUL: &str = "*";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Div`].
pub(crate) const DIV: &str = "/";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Rem`].
pub(crate) const REM: &str = "%";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::And`].
pub(crate) const AND: &str = "&&";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Or`].
pub(crate) const OR: &str = "||";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::BitXor`].
pub(crate) const BIT_XOR: &str = "^";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::BitAnd`].
pub(crate) const BIT_AND: &str = "&";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::BitOr`].
pub(crate) const BIT_OR: &str = "|";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Shl`].
pub(crate) const SHL: &str = "<<";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Shr`].
pub(crate) const SHR: &str = ">>";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Eq`].
pub(crate) const EQ: &str = "==";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Lt`].
pub(crate) const LT: &str = "<";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Le`].
pub(crate) const LE: &str = "<=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Ne`].
pub(crate) const NE: &str = "!=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Ge`].
pub(crate) const GE: &str = ">=";
/// An identifiers used for [`BinOpKind`]'s construction representing a [`BinOpKind::Gt`].
pub(crate) const GT: &str = ">";

/// An identifiers used for [`UnOp`]'s construction representing an [`UnOp::Not`].
pub(crate) const NOT: &str = "!";
/// An identifiers used for [`UnOp`]'s construction representing an [`UnOp::Neg`].
pub(crate) const NEG: &str = "-";
/// An identifiers used for [`UnOp`]'s construction representing an [`UnOp::Com`].
pub(crate) const COM: &str = "~";
/// An identifiers used for [`UnOp`]'s construction representing an [`UnOp::Pos`].
pub(crate) const POS: &str = "+";
/// An identifiers used for [`UnOp`]'s construction representing an [`UnOp::AddrOf`].
pub(crate) const ADDR_OF: &str = "&";
/// An identifiers used for [`UnOp`]'s construction representing an [`UnOp::Deref`].
pub(crate) const DEREF: &str = "*";
