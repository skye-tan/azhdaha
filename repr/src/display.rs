#![allow(clippy::missing_docs_in_private_items)]

use std::fmt::Display;

use itertools::Itertools;

use crate::hir::resolver::SymbolKind;
use crate::hir::{BinOp, Lit, LitKind, PrimTyKind, Ty, TyKind, TyQual, UnOp};
use crate::mir::{
    Body, Const, Local, Operand, Place, Rvalue, Statement, StatementKind, Terminator,
    TerminatorKind,
};

trait MirDisplay {
    fn mir_display(&self, body: &Body) -> String;
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
            writeln!(f, "\n'bb_{}: {{", bb.into_raw())?;

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

impl MirDisplay for Terminator {
    fn mir_display(&self, body: &Body) -> String {
        match &self.kind {
            TerminatorKind::Goto { bb } => {
                format!("goto 'bb_{};", bb.into_raw())
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                let mut result = format!("switch {} {{\n", discr.mir_display(body));

                for (idx, val) in targets.value.iter().enumerate() {
                    result.push_str(&format!(
                        "\t\t{} => 'bb_{};\n",
                        val,
                        targets.bbs.get(idx).unwrap().into_raw()
                    ));
                }

                result.push_str(&format!(
                    "\t\t_ => 'bb_{};\n\t}}",
                    targets.bbs.last().unwrap().into_raw()
                ));

                result
            }
            TerminatorKind::Return => "return;".to_owned(),
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
                format!("{}{}", un_op.mir_display(body), operand.mir_display(body))
            }
            Rvalue::Call(operand, operands) => {
                format!(
                    "{}({})",
                    operand.mir_display(body),
                    operands
                        .iter()
                        .map(|operand| operand.mir_display(body))
                        .join(", ")
                )
            }
            Rvalue::Empty => String::new(),
        }
    }
}

impl MirDisplay for Operand {
    fn mir_display(&self, body: &Body) -> String {
        match self {
            Operand::Place(place) => place.mir_display(body),
            Operand::Const(lit) => match lit {
                Const::Lit(lit) => lit.mir_display(body),
                Const::Symbol(symbol) => match body.symbol_resolver.get_data_by_res(symbol) {
                    SymbolKind::Func(func) => func.ident.name.clone(),
                    SymbolKind::Local(local) => local.ident.name.clone(),
                    SymbolKind::Param(param) => match &param.ident {
                        Some(ident) => ident.name.clone(),
                        None => "unknown".to_owned(),
                    },
                },
            },
        }
    }
}

impl MirDisplay for Place {
    fn mir_display(&self, body: &Body) -> String {
        let result = self.local.mir_display(body);

        for _projection in &self.projections {
            todo!()
        }

        result
    }
}

impl MirDisplay for Local {
    fn mir_display(&self, body: &Body) -> String {
        if let Some(name) = &body.local_decls[*self].debug_name {
            format!("{name}_{}", self.into_raw())
        } else {
            format!("_{}", self.into_raw())
        }
    }
}

impl MirDisplay for Ty {
    fn mir_display(&self, body: &Body) -> String {
        let mut result = String::new();

        if self.is_linear {
            result.push_str("linear");
        }

        for qual in &self.quals {
            result.push_str(&format!("{} ", qual.mir_display(body)));
        }

        result.push_str(&self.kind.mir_display(body));

        result
    }
}

impl MirDisplay for TyKind {
    fn mir_display(&self, body: &Body) -> String {
        match &self {
            TyKind::PrimTy(prim_ty_kind) => prim_ty_kind.mir_display(body),
            TyKind::Ptr { kind, quals } => {
                let mut result = "* ".to_owned();

                for qual in quals {
                    result.push_str(&format!("{} ", qual.mir_display(body)));
                }

                result.push_str(&kind.mir_display(body));

                result
            }
            TyKind::Array { kind, size } => match size {
                Some(_) => format!("[{}; _]", kind.mir_display(body)),
                None => format!("[{}]", kind.mir_display(body)),
            },
            TyKind::Func { .. } => unimplemented!(),
        }
    }
}

impl MirDisplay for PrimTyKind {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            PrimTyKind::Int => "int".to_owned(),
            PrimTyKind::Bool => "bool".to_owned(),
            PrimTyKind::Float => "float".to_owned(),
            PrimTyKind::Double => "double".to_owned(),
            PrimTyKind::Char => "char".to_owned(),
            PrimTyKind::Void => "void".to_owned(),
        }
    }
}

impl MirDisplay for TyQual {
    fn mir_display(&self, _body: &Body) -> String {
        match &self {
            TyQual::Const => "const".to_owned(),
            TyQual::ConstExpr => "constexpr".to_owned(),
            TyQual::Volatile => "volatile".to_owned(),
            TyQual::Restrict => "restrict".to_owned(),
            TyQual::Atomic => "atomic".to_owned(),
            TyQual::NoReturn => "noreturn".to_owned(),
        }
    }
}

impl MirDisplay for Lit {
    fn mir_display(&self, _body: &Body) -> String {
        match &self.kind {
            LitKind::Str(val) => val.to_string(),
            LitKind::Char(val) => val.to_string(),
            LitKind::Int(val) => val.to_string(),
            LitKind::Float(val) => val.to_string(),
        }
    }
}

impl MirDisplay for BinOp {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            BinOp::Add => "+".to_owned(),
            BinOp::Sub => "-".to_owned(),
            BinOp::Mul => "*".to_owned(),
            BinOp::Div => "/".to_owned(),
            BinOp::Rem => "%".to_owned(),
            BinOp::And => "&&".to_owned(),
            BinOp::Or => "||".to_owned(),
            BinOp::BitXor => "^".to_owned(),
            BinOp::BitAnd => "&".to_owned(),
            BinOp::BitOr => "|".to_owned(),
            BinOp::Shl => "<<".to_owned(),
            BinOp::Shr => ">>".to_owned(),
            BinOp::Eq => "==".to_owned(),
            BinOp::Lt => "<".to_owned(),
            BinOp::Le => "<=".to_owned(),
            BinOp::Ne => "!=".to_owned(),
            BinOp::Ge => ">=".to_owned(),
            BinOp::Gt => ">".to_owned(),
            BinOp::Assign => unreachable!(),
        }
    }
}

impl MirDisplay for UnOp {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            UnOp::Not => "!".to_owned(),
            UnOp::Neg => "-".to_owned(),
            UnOp::Com => "~".to_owned(),
            UnOp::Pos => "+".to_owned(),
            UnOp::AddrOf => "&".to_owned(),
            UnOp::Deref => "*".to_owned(),
        }
    }
}
