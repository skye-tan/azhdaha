//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = cli_utils::parse_args();

    let ast_reprs = ast_utils::AstRepr::construct(&args.compile_commands)?;

    if args.dot_graph {
        for (index, ast_repr) in ast_reprs.iter().enumerate() {
            let path = format!("{}.dot", index + 1);

            if let Err(error) = ast_repr.create_dot_graph(&path) {
                log::error!("Failed to create dot-graph file '{path}' - {error:?}");
            }
        }
    }

    println!("{:#?}", hir_repr::HirRepr::lower_ast(&ast_reprs[0])?);

    Ok(())
}
