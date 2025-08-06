//! The analyzer which applies the "Linear Type" rules to the MIR representation of the
//! input source code and reports the possible memory leakages.
//!

use anyhow::Context;
use ariadne::Source;

use repr::mir;

/// Contains methods needed to perform DFS on the MIR.
mod dfs;
/// Contains methods needed to process MIR's components.
mod process;

#[derive(Debug, Clone)]
pub struct LinearLocal {
    pub local: mir::Local,
    pub status: LinearStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinearStatus {
    Owner,
    Free,
}

#[derive(Debug, Clone)]
pub struct LinearCtx<'azhdaha> {
    pub source_path: String,
    pub report_source: Source<&'azhdaha str>,
}

impl<'azhdaha> LinearCtx<'azhdaha> {
    pub fn new(source_path: &str, source_code: &'azhdaha [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            source_path: source_path.to_owned(),
            report_source: Source::from(
                str::from_utf8(source_code)
                    .context("UTF-8 validity for the source code failed.")?,
            ),
        })
    }

    pub fn analyze(&self, body: &mir::Body) {
        let linear_locals: Vec<LinearLocal> = body
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

        for linear_local in linear_locals {
            for (bb, _) in body.basic_blocks.iter() {
                self.dfs_with_stack(body, linear_local.clone(), bb);
            }
        }
    }
}
