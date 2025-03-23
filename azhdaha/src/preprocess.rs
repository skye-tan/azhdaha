use std::process::Command;

use anyhow::Context;
use compile_commands::{CompilationDatabase, CompileArgs};

/// The flag which indicates that only preprocess phase should be done.
const PREPROCESS_ONLY_FLAG: &str = "-E";

/// The flag used for including headers.
#[allow(dead_code)]
const INCLUDE_FLAG: &str = "-I";

/// Replace headers with annotated versions and expand macros.
///
/// In order to only perform the preprocessing on the source code, [`PREPROCESS_ONLY_FLAG`] flag
/// is appended to the arguments.
///
/// Headers used in the source code must be replaced with annotated versions which is accomplished by
/// inserting [`INCLUDE_FLAG`] flag pointing to a directory containing the annotated headers.
///
/// The LINEAR macro used in the source codes must be replaced by `_Linear` type qualifier which is accomplished by
/// inserting [`INCLUDE_FLAG`] flag pointing to a directory containing the edited header.
///
pub fn expand(compile_commands: &CompilationDatabase) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut results = vec![];

    for compile_command in compile_commands {
        let directory = compile_command
            .directory
            .to_str()
            .context("UTF-8 validity failed for compile-commands.")?;

        if !compile_command.directory.exists() {
            log::error!(
                "Directory '{}' used in compile-commands does not exist.",
                directory
            )
        }

        let (command, args) = match compile_command
            .arguments
            .as_ref()
            .context("Arguments section was not found in compile-commands.")?
        {
            CompileArgs::Arguments(args) => (args[0].clone(), args[1..].to_vec()),
            CompileArgs::Flags(_) => {
                // Arguments might be read from compile-flags which is not currently supported.
                panic!("Compile-flags is not currently supported.")
            }
        };

        results.push(
            Command::new(command)
                .args(args)
                .arg(PREPROCESS_ONLY_FLAG)
                .current_dir(directory)
                .output()?
                .stdout,
        );
    }

    Ok(results)
}
