use std::fmt::Display;

use crate::{Body, Operand, Place, Rvalue, Statement, StatementKind, Terminator, TerminatorKind};

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Place(place) => write!(f, "{place}"),
            Operand::Constant(const_operand) => todo!(),
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
        write!(f, "'{}", self.local.into_raw())?;

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
            writeln!(f, "let '{}: {};", local.into_raw(), local_decl.ty)?;
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
