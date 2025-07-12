use std::fmt::Display;

use crate::hir::{BinOp, Lit, LitKind, PrimTyKind, Ty, TyKind, UnOp};
use crate::mir::{
    Body, Operand, Place, Rvalue, Statement, StatementKind, Terminator, TerminatorKind,
};

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            LitKind::Str(val) => write!(f, "const {val} char*"),
            LitKind::Char(val) => write!(f, "const {val} char"),
            LitKind::Int(val) => write!(f, "const {val} int"),
            LitKind::Float(val) => write!(f, "const {val} float"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Rem => write!(f, "%"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::BitXor => write!(f, "^"),
            BinOp::BitAnd => write!(f, "&"),
            BinOp::BitOr => write!(f, "|"),
            BinOp::Shl => write!(f, "<<"),
            BinOp::Shr => write!(f, ">>"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Ge => write!(f, ">="),
            BinOp::Gt => write!(f, ">"),
            BinOp::Assign => unreachable!(),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Not => write!(f, "!"),
            UnOp::Neg => write!(f, "-"),
            UnOp::Com => write!(f, "~"),
            UnOp::Pos => write!(f, "+"),
            UnOp::AddrOf => write!(f, "&"),
            UnOp::Deref => write!(f, "*"),
        }
    }
}

impl Display for PrimTyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimTyKind::Int => write!(f, "int"),
            PrimTyKind::Float => write!(f, "float"),
            PrimTyKind::Double => write!(f, "double"),
            PrimTyKind::Char => write!(f, "char"),
            PrimTyKind::Void => write!(f, "void"),
        }
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

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Place(place) => write!(f, "{place}"),
            Operand::Const(lit) => write!(f, "{lit}"),
        }
    }
}

impl Display for Rvalue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rvalue::Use(operand) => write!(f, "{operand}"),
            Rvalue::BinaryOp(bin_op, left_operand, right_operand) => {
                write!(f, "{} {} {}", left_operand, bin_op, right_operand)
            }
            Rvalue::UnaryOp(un_op, operand) => write!(f, "{un_op} {operand}"),
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{}", self.local.into_raw())?;

        for projection in &self.projections {
            todo!()
        }

        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StatementKind::Assign(place, rvalue) => {
                write!(f, "{place} = {rvalue}")
            }
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TerminatorKind::Goto { target } => todo!(),
            TerminatorKind::SwitchInt { discr, targets } => todo!(),
            TerminatorKind::Return => write!(f, "return;"),
        }
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (local, local_decl) in self.local_decls.iter() {
            writeln!(f, "let _{}: {};", local.into_raw(), local_decl.ty)?;
        }

        for (bb, bb_data) in self.basic_blocks.iter() {
            writeln!(f, "\nbb-{}: {{", bb.into_raw())?;

            for stmt in &bb_data.statements {
                writeln!(f, "\t{stmt}")?;
            }

            if let Some(terminator) = &bb_data.terminator {
                writeln!(f, "\t{terminator}")?;
            }

            writeln!(f, "}}")?;
        }

        Ok(())
    }
}
