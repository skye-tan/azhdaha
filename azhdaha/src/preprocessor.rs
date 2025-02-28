use std::process::Command;

use anyhow::Context;
use compile_commands::{CompilationDatabase, CompileArgs};

/// A flag which indicates that only preprocess should be executed.
const PREPROCESS_ONLY_FLAG: &str = "-E";

/// Replace headers with annotated versions and expand macros.
///
/// In order to only perform the preprocessing on the source code, [`PREPROCESS_ONLY_FLAG`] flag
/// is appended to the arguments.
///
/// Headers used in the source code must be replaced with annotated versions which is accomplished by
/// inserting -I flag pointing to a directory containing the annotated headers.
///
/// The LINEAR macro used in the source codes must be replaced by `_Linear` type qualifier which is accomplished by
/// inserting -I flag pointing to a directory containing the edited header.
///
pub fn expand(compile_commands: &CompilationDatabase) -> anyhow::Result<Vec<String>> {
    let mut results = vec![];

    for compile_command in compile_commands {
        let directory = compile_command
            .directory
            .to_str()
            .context("Invalid directory in compile-commands.")?;

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

        results.push(String::from_utf8(
            Command::new(command)
                .args(args)
                .arg(PREPROCESS_ONLY_FLAG)
                .current_dir(directory)
                .output()?
                .stdout,
        )?);
    }

    Ok(results)
}
