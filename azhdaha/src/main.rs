//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

use ast_utils::AstRepr;

use repr::{
    hir::{self, LoweringCtx},
    mir::MirCtx,
};

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    env_logger::init();

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

    let lowering_ctx = LoweringCtx::lower_ast(&ast_reprs[0]);
    println!(
        "\n{:#?}\n{:#?}\n",
        lowering_ctx.items, lowering_ctx.resolver
    );

    for item in lowering_ctx.items {
        match item.kind {
            hir::ItemKind::Fn(f) => {
                let ctx = MirCtx::new(&lowering_ctx.resolver, f.body.span);
                let mir_body = ctx.lower(&f);

                if let Some(mir_body) = mir_body {
                    println!("\n{mir_body}");
                }
            }
            hir::ItemKind::Union => todo!(),
            hir::ItemKind::Struct => todo!(),
            hir::ItemKind::GlobalVar => todo!(),
        }
    }

    Ok(())
}
