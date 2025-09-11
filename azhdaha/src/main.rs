//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.
//!

use std::io::Write;

use analyzer::LinearCtx;
use ast_utils::AstRepr;
use log::error;

use env_logger::Env;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use repr::{
    hir::{HirCtx, ItemKind},
    mir::MirCtx,
};

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
        .format_source_path(true)
        .init();

    let args = cli_utils::parse_args();

    let ast_reprs = AstRepr::construct(&args.compile_commands)?;

    if args.dot_graph {
        for (index, ast_repr) in ast_reprs.iter().enumerate() {
            let path = format!("{}.dot", index + 1);

            if let Err(error) = ast_repr.create_dot_graph(&path) {
                error!("Failed to create dot-graph for '{path}' - {error:?}");
            }
        }
    }

    for ast_repr in ast_reprs {
        let hir_ctx = HirCtx::new(&ast_repr);

        let items = hir_ctx.lower_to_hir();

        let linear_ctx = LinearCtx::new(&ast_repr.source_info.path, &ast_repr.source_info.code)?;

        let reports: Vec<Vec<u8>> = items
            .into_par_iter()
            .filter_map(|item| {
                let ItemKind::Func(func_def) = item.kind else {
                    return None;
                };

                let mir_ctx = MirCtx::new(
                    &func_def.symbol_resolver,
                    &func_def.label_resolver,
                    func_def.body.span,
                );

                let mir_body = mir_ctx.lower_to_mir(&func_def);

                match mir_body {
                    Ok(mir_body) => linear_ctx.analyze(&mir_body),
                    Err(error) => {
                        error!("Failed to construct MIR - {error:?}");

                        None
                    }
                }
            })
            .collect();

        if reports.is_empty() {
            println!(
                "Entry \"{}\" was analyzed successfully.",
                ast_repr.source_info.path
            );
        } else {
            println!(
                "Entry \"{}\" was found to be problematic.",
                ast_repr.source_info.path
            );
        }

        if !args.do_not_report {
            for report in reports {
                std::io::stdout().write_all(&report)?;
            }
        }
    }

    Ok(())
}
