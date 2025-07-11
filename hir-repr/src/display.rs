use std::fmt::Display;

use crate::{BinOp, PrimTyKind, Ty, TyKind, UnOp};

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::BitXor => "^",
            BinOp::BitAnd => "&",
            BinOp::BitOr => "|",
            BinOp::Shl => "<<",
            BinOp::Shr => ">>",
            BinOp::Eq => "==",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Ne => "!=",
            BinOp::Ge => ">=",
            BinOp::Gt => ">",
            BinOp::Assign => unreachable!(),
        };

        write!(f, "{str}")
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UnOp::Not => "!",
            UnOp::Neg => "-",
            UnOp::Com => "~",
            UnOp::Pos => "+",
            UnOp::AddrOf => "&",
            UnOp::Deref => "*",
        };

        write!(f, "{str}")
    }
}

impl Display for PrimTyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PrimTyKind::Int => "int",
            PrimTyKind::Float => "float",
            PrimTyKind::Double => "double",
            PrimTyKind::Char => "char",
            PrimTyKind::Void => "void",
        };

        write!(f, "{str}")
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TyKind::PrimTy(prim_ty_kind) => write!(f, "{prim_ty_kind}"),
            TyKind::Array(ty, _) => write!(f, "[{ty}]"),
            TyKind::Ptr(ty) => write!(f, "*{ty}"),
        }
    }
}
