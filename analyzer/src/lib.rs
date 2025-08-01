//! The analyzer which applies the "Linear Type" rules of the MIR representation of the
//! input source code and reports the possible memory leakages.
//!

use anyhow::bail;
use ariadne::{ColorGenerator, Fmt, Label, Report, ReportKind, Source};

use repr::mir;

#[derive(Debug, Clone)]
pub struct LinearCtx<'azhdaha> {
    source_path: String,
    report_source: Source<&'azhdaha str>,
}

#[derive(Debug, Clone)]
struct LinearLocal {
    local: mir::Local,
    status: LinearStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LinearStatus {
    Owner,
    Free,
}

impl<'azhdaha> LinearCtx<'azhdaha> {
    pub fn new(source_path: &str, source_code: &'azhdaha [u8]) -> Self {
        Self {
            source_path: source_path.to_owned(),
            report_source: Source::from(str::from_utf8(source_code).unwrap()),
        }
    }

    pub fn analyze(&self, body: &mir::Body) -> anyhow::Result<()> {
        let linear_decls: Vec<LinearLocal> = body
            .local_decls
            .iter()
            .filter_map(|(local, local_decl)| {
                if local_decl.ty.is_linear {
                    return Some(LinearLocal {
                        local,
                        status: LinearStatus::Free,
                    });
                }

                None
            })
            .collect();

        for local in linear_decls {
            for (bb, _) in body.basic_blocks.iter() {
                self.dfs(body, local.clone(), bb)?;
            }
        }

        Ok(())
    }

    fn dfs(
        &self,
        body: &mir::Body,
        mut linear_local: LinearLocal,
        bb: mir::BasicBlock,
    ) -> anyhow::Result<()> {
        let bb_data = &body.basic_blocks[bb];

        for stmt in &bb_data.statements {
            let mut is_used = false;

            match &stmt.kind {
                mir::StatementKind::Assign(place, rvalue) => {
                    match rvalue {
                        mir::Rvalue::Use(operand) => {
                            if let mir::Operand::Place(place) = operand {
                                if place.local == linear_local.local {
                                    is_used = true;
                                }
                            }
                        }
                        mir::Rvalue::BinaryOp(_, left_op, right_op) => {
                            if let mir::Operand::Place(place) = left_op {
                                if place.local == linear_local.local {
                                    is_used = true;
                                }
                            }

                            if let mir::Operand::Place(place) = right_op {
                                if place.local == linear_local.local {
                                    is_used = true;
                                }
                            }
                        }
                        mir::Rvalue::UnaryOp(_, operand) => {
                            if let mir::Operand::Place(place) = operand {
                                if place.local == linear_local.local {
                                    is_used = true;
                                }
                            }
                        }
                        mir::Rvalue::Call(operand, operands) => {
                            if let mir::Operand::Place(place) = operand {
                                if place.local == linear_local.local {
                                    is_used = true;
                                }
                            }

                            for operand in operands {
                                if let mir::Operand::Place(place) = operand {
                                    if place.local == linear_local.local {
                                        is_used = true;
                                    }
                                }
                            }
                        }
                        mir::Rvalue::Empty => (),
                    }

                    let lhs = &body.local_decls[place.local];

                    if lhs.ty.is_linear {
                        if is_used {
                            if linear_local.status == LinearStatus::Free {
                                let mut colors = ColorGenerator::new();
                                let a = colors.next();

                                Report::build(
                                    ReportKind::Error,
                                    (&self.source_path, stmt.span.lo..stmt.span.hi),
                                )
                                .with_message(format!("Use of moved value"))
                                .with_label(
                                    Label::new((&self.source_path, place.span.lo..place.span.hi))
                                        .with_message(format!(
                                            "This is declared as {}",
                                            "linear".fg(a)
                                        ))
                                        .with_color(a),
                                )
                                .finish()
                                .print((&self.source_path, &self.report_source))
                                .unwrap();

                                bail!("");
                            }

                            linear_local.status = LinearStatus::Free;
                        }
                    } else if is_used {
                        if linear_local.status == LinearStatus::Free {
                            let mut colors = ColorGenerator::new();
                            let a = colors.next();

                            Report::build(
                                ReportKind::Error,
                                (&self.source_path, stmt.span.lo..stmt.span.hi),
                            )
                            .with_message(format!("Use of moved value"))
                            .with_label(
                                Label::new((&self.source_path, place.span.lo..place.span.hi))
                                    .with_message(format!(
                                        "This is declared as {}",
                                        "non-linear".fg(a)
                                    ))
                                    .with_color(a),
                            )
                            .finish()
                            .print((&self.source_path, &self.report_source))
                            .unwrap();

                            bail!("");
                        }
                    }

                    if place.local == linear_local.local {
                        if linear_local.status == LinearStatus::Owner {
                            let mut colors = ColorGenerator::new();
                            let a = colors.next();

                            Report::build(
                                ReportKind::Error,
                                (&self.source_path, stmt.span.lo..stmt.span.hi),
                            )
                            .with_message(format!("Overwriting owned value"))
                            .with_label(
                                Label::new((&self.source_path, place.span.lo..place.span.hi))
                                    .with_message(format!(
                                        "This is declared as {} and already owns a value.",
                                        "linear".fg(a)
                                    ))
                                    .with_color(a),
                            )
                            .finish()
                            .print((&self.source_path, &self.report_source))
                            .unwrap();

                            bail!("");
                        }

                        linear_local.status = LinearStatus::Owner;
                    }
                }
                mir::StatementKind::Rvalue(_rvalue) => {}
            }
        }

        let Some(terminator) = &bb_data.terminator else {
            if linear_local.status == LinearStatus::Owner {
                bail!(
                    "Variable {} might not move its value before the end.",
                    linear_local.local.into_raw(),
                )
            }

            return Ok(());
        };

        match &terminator.kind {
            mir::TerminatorKind::Goto { bb } => self.dfs(body, linear_local, *bb),
            mir::TerminatorKind::SwitchInt { targets, .. } => {
                for target in &targets.bbs {
                    self.dfs(body, linear_local.clone(), *target)?;
                }

                Ok(())
            }
            mir::TerminatorKind::Return => {
                if linear_local.status == LinearStatus::Owner {
                    bail!(
                        "Variable {} might not move its value before the end.",
                        linear_local.local.into_raw(),
                    )
                }
                Ok(())
            }
        }
    }
}
