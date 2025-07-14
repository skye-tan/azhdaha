use std::process::Command;

use compile_commands::{CompilationDatabase, CompileArgs};

/// The flag which indicates that only preprocess phase should be done.
const PREPROCESS_ONLY_FLAG: &str = "-E";

/// The flag which inhibits generation of linemarkers in the output from the preprocessor.
const INHABIT_LINEMARKS_FLAG: &str = "-P";

/// The flag which is used to include headers.
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
pub(crate) fn preprocess(compile_commands: &CompilationDatabase) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut results = vec![];

    for compile_command in compile_commands {
        let Some(directory) = compile_command.directory.to_str() else {
            log::warn!("UTF-8 validity failed.");
            continue;
        };

        if !compile_command.directory.exists() {
            log::warn!("Directory '{}' does not exist.", directory);
            continue;
        }

        let Some(arguments) = compile_command.arguments.as_ref() else {
            log::warn!("Arguments section was not found.");
            continue;
        };

        let (command, args) = match arguments {
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
                .arg(INHABIT_LINEMARKS_FLAG)
                .current_dir(directory)
                .output()?
                .stdout,
        );
    }

    Ok(results)
}
