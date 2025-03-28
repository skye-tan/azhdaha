//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = cli_utils::parse_args();

    let asts = ast_utils::AST::construct(&args.compile_commands)?;

    if args.dot_graph {
        for (index, ast) in asts.iter().enumerate() {
            let path = format!("{index}.dot");

            if let Err(error) = ast.create_dot_graph(&path) {
                log::error!("Failed to create dot graph file '{path}' with error: {error:?}");
            }
        }
    }

    println!("{:#?}", hir_repr::construct_hir(&asts[0])?);

    Ok(())
}
