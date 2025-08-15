#![allow(clippy::missing_docs_in_private_items)]

use anyhow::Context;
use ariadne::Source;
use la_arena::{Idx, RawIdx};
use log::error;

use repr::{hir::Span, mir};

use crate::report::ReportCache;

#[derive(Debug, Clone)]
pub(crate) struct LinearLocal {
    pub(crate) name: String,
    pub(crate) local: mir::Local,
    pub(crate) status: LinearStatus,
    pub(crate) is_altered: bool,
    pub(crate) span: Span,
}

impl LinearLocal {
    pub(crate) fn set_free(&mut self) {
        if matches!(self.status, LinearStatus::Owner) {
            self.is_altered = true;
        }

        self.status = LinearStatus::Free;
    }

    pub(crate) fn set_owner(&mut self) {
        if matches!(self.status, LinearStatus::Free) {
            self.is_altered = true;
        }

        self.status = LinearStatus::Owner;
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LinearStatus {
    Owner,
    Free,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LinearCtx<'linear> {
    pub(crate) source_path: String,
    pub(crate) report_source: Source<&'linear str>,
}

impl<'linear> LinearCtx<'linear> {
    pub fn new(source_path: &str, source_code: &'linear [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            source_path: source_path.to_owned(),
            report_source: Source::from(
                str::from_utf8(source_code)
                    .context("UTF-8 validity for the source code failed.")?,
            ),
        })
    }

    pub fn analyze(&self, body: &mir::Body) {
        let mut linear_locals: Vec<LinearLocal> = body
            .local_decls
            .iter()
            .filter_map(|(local, local_decl)| match &local_decl.kind {
                mir::LocalKind::Real { ty, ident, .. } => {
                    if ty.is_linear {
                        Some(LinearLocal {
                            name: ident.name.clone(),
                            local,
                            status: LinearStatus::Unknown,
                            is_altered: false,
                            span: local_decl.span,
                        })
                    } else {
                        None
                    }
                }
                mir::LocalKind::Temp => None,
            })
            .collect();

        // TODO: Must be removed in the future.
        linear_locals.push(LinearLocal {
            name: "dummy".to_owned(),
            local: Idx::from_raw(RawIdx::from_u32(u32::MAX)),
            status: LinearStatus::Free,
            is_altered: false,
            span: repr::hir::Span { lo: 0, hi: 0 },
        });

        for linear_local in linear_locals {
            for (bb, _) in body.basic_blocks.iter() {
                if let Some(report) = self.dfs_with_stack(body, linear_local.clone(), bb.into()) {
                    if let Err(error) = report.print(ReportCache::new(
                        self.source_path.clone(),
                        &self.report_source,
                    )) {
                        error!("Failed to print the linear analyzer's report - {error:?}");
                    }

                    break;
                }
            }
        }
    }
}
