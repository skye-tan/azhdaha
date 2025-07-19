//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

use ast_utils::AstRepr;

use repr::{
    hir::{self, HirCtx},
    mir::MirCtx,
};

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    env_logger::builder().format_source_path(true).init();

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

    let hir_ctx = HirCtx::new(&ast_reprs[0]);

    let items = hir_ctx.lower_to_hir();

    println!("{}", items.len());

    // println!("\n{:#?}\n", items);

    for item in items {
        match item.kind {
            hir::ItemKind::Func(func) => {
                let mir_ctx =
                    MirCtx::new(&func.symbol_resolver, &func.label_resolver, func.body.span);

                let mir_body = mir_ctx.lower_to_mir(&func);

                match mir_body {
                    Ok(mir_body) => println!("\n{mir_body}"),
                    Err(error) => println!("\nFailed to construct mir - {error:?}"),
                }
            }
            _ => continue,
        }
    }

    Ok(())
}
