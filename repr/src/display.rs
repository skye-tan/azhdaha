use std::fmt::Display;

use itertools::Itertools;

use crate::hir::{BinOp, Lit, LitKind, PrimTyKind, Ty, TyKind, UnOp};
use crate::mir::{
    Body, Local, Operand, Place, Rvalue, Statement, StatementKind, Terminator, TerminatorKind,
};

trait MirDisplay {
    fn mir_display(&self, body: &Body) -> String;
}

impl MirDisplay for Lit {
    fn mir_display(&self, _body: &Body) -> String {
        match &self.kind {
            LitKind::Str(val) => format!("{val}"),
            LitKind::Char(val) => format!("{val}"),
            LitKind::Int(val) => format!("{val}"),
            LitKind::Float(val) => format!("{val}"),
        }
    }
}

impl MirDisplay for BinOp {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            BinOp::Add => format!("+"),
            BinOp::Sub => format!("-"),
            BinOp::Mul => format!("*"),
            BinOp::Div => format!("/"),
            BinOp::Rem => format!("%"),
            BinOp::And => format!("&&"),
            BinOp::Or => format!("||"),
            BinOp::BitXor => format!("^"),
            BinOp::BitAnd => format!("&"),
            BinOp::BitOr => format!("|"),
            BinOp::Shl => format!("<<"),
            BinOp::Shr => format!(">>"),
            BinOp::Eq => format!("=="),
            BinOp::Lt => format!("<"),
            BinOp::Le => format!("<="),
            BinOp::Ne => format!("!="),
            BinOp::Ge => format!(">="),
            BinOp::Gt => format!(">"),
            BinOp::Assign => unreachable!(),
        }
    }
}

impl MirDisplay for UnOp {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            UnOp::Not => format!("!"),
            UnOp::Neg => format!("-"),
            UnOp::Com => format!("~"),
            UnOp::Pos => format!("+"),
            UnOp::AddrOf => format!("&"),
            UnOp::Deref => format!("*"),
        }
    }
}

impl MirDisplay for PrimTyKind {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            PrimTyKind::Int => format!("int"),
            PrimTyKind::Float => format!("float"),
            PrimTyKind::Double => format!("double"),
            PrimTyKind::Char => format!("char"),
            PrimTyKind::Void => format!("void"),
        }
    }
}

impl MirDisplay for Ty {
    fn mir_display(&self, body: &Body) -> String {
        match &self.kind {
            TyKind::PrimTy(prim_ty_kind) => format!("{}", prim_ty_kind.mir_display(body)),
            TyKind::Array(ty, _) => format!("[{}]", ty.mir_display(body)),
            TyKind::Ptr(ty) => format!("*{}", ty.mir_display(body)),
        }
    }
}

impl MirDisplay for Local {
    fn mir_display(&self, body: &Body) -> String {
        if let Some(name) = &body.local_decls[*self].debug_ident {
            format!("{name}_{}", self.into_raw())
        } else {
            format!("_{}", self.into_raw())
        }
    }
}

impl MirDisplay for Place {
    fn mir_display(&self, body: &Body) -> String {
        let result = self.local.mir_display(body);

        for projection in &self.projections {
            todo!()
        }

        result
    }
}

impl MirDisplay for Operand {
    fn mir_display(&self, body: &Body) -> String {
        match self {
            Operand::Place(place) => place.mir_display(body),
            Operand::Const(lit) => match lit {
                crate::mir::Const::Lit(lit) => lit.mir_display(body),
                crate::mir::Const::Fn(res) => body.resolver.get_item(res).ident.name.clone(),
            },
        }
    }
}

impl MirDisplay for Rvalue {
    fn mir_display(&self, body: &Body) -> String {
        match self {
            Rvalue::Use(operand) => operand.mir_display(body),
            Rvalue::BinaryOp(bin_op, left_operand, right_operand) => {
                format!(
                    "{} {} {}",
                    left_operand.mir_display(body),
                    bin_op.mir_display(body),
                    right_operand.mir_display(body)
                )
            }
            Rvalue::UnaryOp(un_op, operand) => {
                format!("{} {}", un_op.mir_display(body), operand.mir_display(body))
            }
            Rvalue::Call(operand, operands) => {
                format!(
                    "{}({})",
                    operand.mir_display(body),
                    operands.iter().map(|x| x.mir_display(body)).join(", ")
                )
            }
        }
    }
}

impl MirDisplay for Statement {
    fn mir_display(&self, body: &Body) -> String {
        match &self.kind {
            StatementKind::Assign(place, rvalue) => {
                format!("{} = {}", place.mir_display(body), rvalue.mir_display(body))
            }
        }
    }
}

impl MirDisplay for Terminator {
    fn mir_display(&self, _body: &Body) -> String {
        match &self.kind {
            TerminatorKind::Goto { bb } => todo!(),
            TerminatorKind::SwitchInt { discr, targets } => todo!(),
            TerminatorKind::Return => "return;".to_owned(),
        }
    }
}

impl Display for Body<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (local, local_decl) in self.local_decls.iter() {
            writeln!(
                f,
                "let {}: {};",
                local.mir_display(self),
                local_decl.ty.mir_display(self)
            )?;
        }

        for (bb, bb_data) in self.basic_blocks.iter() {
            writeln!(f, "\nbb-{}: {{", bb.into_raw())?;

            for stmt in &bb_data.statements {
                writeln!(f, "\t{};", stmt.mir_display(self))?;
            }

            if let Some(terminator) = &bb_data.terminator {
                writeln!(f, "\t{}", terminator.mir_display(self))?;
            }

            writeln!(f, "}}")?;
        }

        Ok(())
    }
}
