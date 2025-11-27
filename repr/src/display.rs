#![allow(clippy::missing_docs_in_private_items)]

use std::fmt::Display;

use itertools::Itertools;

use crate::hir::resolver::SymbolKind;
use crate::hir::{Lit, LitKind, PrimTyKind, Storage, Ty, TyKind, TyQual};
use crate::mir::{
    Body, Const, IntBinOp, IntUnOp, Local, LocalKind, Operand, Place, PlaceElem, Rvalue, Statement,
    StatementKind, Terminator, TerminatorKind,
};

trait MirDisplay {
    fn mir_display(&self, body: &Body) -> String;
}

impl Display for Body<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (local, local_decl) in self.local_decls.iter() {
            match &local_decl.kind {
                LocalKind::Real {
                    storage,
                    ident,
                    is_arg,
                } => {
                    write!(formatter, "let {}_{}", ident.name, local.into_raw())?;

                    if *is_arg {
                        write!(formatter, "(arg)")?;
                    }

                    write!(formatter, ":")?;

                    if let Some(storage) = storage {
                        write!(formatter, " {}", storage.mir_display(self))?;
                    }

                    writeln!(formatter, " {};", local_decl.ty.mir_display(self))?;
                }
                LocalKind::Temp => {
                    writeln!(
                        formatter,
                        "let {}: {};",
                        local.mir_display(self),
                        local_decl.ty.mir_display(self)
                    )?;
                }
            }
        }

        for (bb, bb_data) in self.basic_blocks.iter() {
            writeln!(formatter, "\n'bb_{}: {{", bb.into_raw())?;

            for stmt in &bb_data.statements {
                writeln!(formatter, "\t{};", stmt.mir_display(self))?;
            }

            if let Some(terminator) = &bb_data.terminator {
                writeln!(formatter, "\t{}", terminator.mir_display(self))?;
            }

            writeln!(formatter, "}}")?;
        }

        Ok(())
    }
}

impl MirDisplay for Terminator {
    fn mir_display(&self, body: &Body) -> String {
        match &self.kind {
            TerminatorKind::Goto { bb } => {
                format!("goto 'bb_{};", bb.get_id())
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                let mut result = format!("switch {} {{\n", discr.mir_display(body));

                result.push_str(&format!("\t\t1 => 'bb_{};\n", targets[0].get_id()));

                result.push_str(&format!("\t\t_ => 'bb_{};\n\t}}", targets[1].get_id()));

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
            StatementKind::Call(operand, operands) => format!(
                "{}({})",
                operand.mir_display(body),
                operands
                    .iter()
                    .map(|operand| operand.mir_display(body))
                    .join(", ")
            ),
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
            Rvalue::PtrDiff(left_operand, right_operand) => {
                format!(
                    "{} - {} // (ptr diff)",
                    left_operand.mir_display(body),
                    right_operand.mir_display(body)
                )
            }

            Rvalue::UnaryOp(un_op, operand) => {
                format!("{}{}", un_op.mir_display(body), operand.mir_display(body))
            }
            Rvalue::AddrOf(place) => {
                format!("&{}", place.mir_display(body))
            }
            Rvalue::AddrOfStatic(symbol) => {
                format!("&<todo {symbol:?}>")
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
            Rvalue::Cast {
                value,
                from_type,
                to_type,
            } => {
                format!(
                    "({} -> {}) {}",
                    from_type.mir_display(body),
                    to_type.mir_display(body),
                    value.mir_display(body),
                )
            }
            Rvalue::CompoundInitializing(_, _) => "initializer tree".to_owned(),
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
                Const::Symbol(symbol) => {
                    let symbol_kind = body.symbol_resolver.get_data_by_res(symbol);

                    match symbol_kind {
                        SymbolKind::Func(func) => func.ident.name.clone(),
                        SymbolKind::Var(local) => local.ident.name.clone(),
                        SymbolKind::Param(param) => match &param.ident {
                            Some(ident) => ident.name.clone(),
                            None => "unknown".to_owned(),
                        },
                        SymbolKind::TyDef(ty) => ty.mir_display(body),
                        SymbolKind::EnumVariant { value, span: _ } => format!("Enum({value})"),
                    }
                }
                Const::Sizeof(ty) => format!("sizeof({})", ty.mir_display(body)),
            },
        }
    }
}

impl MirDisplay for Place {
    fn mir_display(&self, body: &Body) -> String {
        let mut result = String::new();

        for projection in &self.projections {
            if matches!(projection, PlaceElem::Deref) {
                result.push_str(&projection.mir_display(body));
            }
        }

        result.push_str(&self.local.mir_display(body));

        for projection in &self.projections {
            if !matches!(projection, PlaceElem::Deref) {
                result.push_str(&projection.mir_display(body));
            }
        }

        result
    }
}

impl MirDisplay for PlaceElem {
    fn mir_display(&self, body: &Body) -> String {
        match self {
            PlaceElem::Field(field) => format!(".{field}"),
            PlaceElem::Index(place) => format!("[{}]", place.mir_display(body)),
            PlaceElem::Deref => "*".to_owned(),
        }
    }
}

impl MirDisplay for Local {
    fn mir_display(&self, body: &Body) -> String {
        match &body.local_decls[*self].kind {
            LocalKind::Real { ident, .. } => format!("{}_{}", ident.name, self.into_raw()),
            LocalKind::Temp => format!("_{}", self.into_raw()),
        }
    }
}

impl MirDisplay for Ty {
    fn mir_display(&self, body: &Body) -> String {
        let mut result = String::new();

        if self.is_linear {
            result.push_str("linear ");
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
            TyKind::Struct(ident) => format!("struct /*todo {ident:?}*/"),
            TyKind::Union(ident) => format!("union /*todo {ident:?}*/"),
            TyKind::Ptr { kind, quals } => {
                let mut result = String::new();

                result.push_str(&kind.mir_display(body));

                result.push('*');

                for qual in quals {
                    result.push_str(&format!("{} ", qual.mir_display(body)));
                }

                result
            }
            TyKind::Array { kind, .. } => format!("{}[]", kind.mir_display(body)),
            TyKind::Func { .. } => "function pointer".to_owned(),
            TyKind::VaList => "va_list".to_owned(),
            TyKind::InitializerList => "initializer list".to_owned(),
        }
    }
}

impl MirDisplay for PrimTyKind {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            PrimTyKind::Int(bytes) => format!("i{}", *bytes * 8),
            PrimTyKind::Bool => "bool".to_owned(),
            PrimTyKind::Float(bytes) => format!("f{}", *bytes * 8),
            PrimTyKind::Char => "char".to_owned(),
            PrimTyKind::Void => "void".to_owned(),
        }
    }
}

impl MirDisplay for Storage {
    fn mir_display(&self, _body: &Body) -> String {
        match &self {
            Storage::Extern => "extern".to_owned(),
            Storage::Static => "static".to_owned(),
            Storage::Auto => "auto".to_owned(),
            Storage::Register => "register".to_owned(),
            Storage::Inline => "inline".to_owned(),
            Storage::ThreadLocal => "thread_local".to_owned(),
        }
    }
}

impl MirDisplay for TyQual {
    fn mir_display(&self, _body: &Body) -> String {
        match &self {
            TyQual::Const => "const",
            TyQual::ConstExpr => "constexpr",
            TyQual::Volatile => "volatile",
            TyQual::Restrict => "restrict",
            TyQual::Atomic => "atomic",
            TyQual::NoReturn => "noreturn",
            TyQual::Extension => "__extension__",
        }
        .to_owned()
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

impl MirDisplay for IntBinOp {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            IntBinOp::Add => "+".to_owned(),
            IntBinOp::Sub => "-".to_owned(),
            IntBinOp::Mul => "*".to_owned(),
            IntBinOp::Div => "/".to_owned(),
            IntBinOp::Rem => "%".to_owned(),
            IntBinOp::BitXor => "^".to_owned(),
            IntBinOp::BitAnd => "&".to_owned(),
            IntBinOp::BitOr => "|".to_owned(),
            IntBinOp::Shl => "<<".to_owned(),
            IntBinOp::Shr => ">>".to_owned(),
            IntBinOp::Eq => "==".to_owned(),
            IntBinOp::Lt => "<".to_owned(),
            IntBinOp::Le => "<=".to_owned(),
            IntBinOp::Ne => "!=".to_owned(),
            IntBinOp::Ge => ">=".to_owned(),
            IntBinOp::Gt => ">".to_owned(),
        }
    }
}

impl MirDisplay for IntUnOp {
    fn mir_display(&self, _body: &Body) -> String {
        match self {
            IntUnOp::Not => "!".to_owned(),
            IntUnOp::Neg => "-".to_owned(),
            IntUnOp::Com => "~".to_owned(),
            IntUnOp::Pos => "+".to_owned(),
        }
    }
}
