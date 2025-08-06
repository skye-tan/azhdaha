//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.
//!

use analyzer::LinearCtx;
use ast_utils::AstRepr;

use env_logger::Env;
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
                log::error!("Failed to create dot-graph for '{path}' - {error:?}");
            }
        }
    }

    for ast_repr in ast_reprs {
        let hir_ctx = HirCtx::new(&ast_repr);

        let items = hir_ctx.lower_to_hir();

        let linear_ctx = LinearCtx::new(&ast_repr.source_info.path, &ast_repr.source_info.code)?;

        for item in items {
            match item.kind {
                ItemKind::Func(func_def) => {
                    let mir_ctx = MirCtx::new(
                        &func_def.symbol_resolver,
                        &func_def.label_resolver,
                        func_def.body.span,
                    );

                    let mir_body = mir_ctx.lower_to_mir(&func_def);

                    match mir_body {
                        Ok(mir_body) => {
                            println!("{mir_body}");

                            linear_ctx.analyze(&mir_body)
                        }
                        Err(error) => println!("\nFailed to construct MIR - {error:?}"),
                    }
                }
                _ => continue,
            }
        }
    }

    Ok(())
}
