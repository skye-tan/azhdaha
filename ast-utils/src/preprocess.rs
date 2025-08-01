use std::process::Command;

use compile_commands::{CompilationDatabase, CompileArgs};

/// Indicates that only preprocess phase should be done.
const PREPROCESS_ONLY_FLAG: &str = "-E";

/// Inhibits generation of linemarkers in the output from the preprocessor.
const INHABIT_LINEMARKS_FLAG: &str = "-P";

/// Includes annotated headers.
const INCLUDE_FLAG: &str = "-I";

/// The directory which contains the annotated headers.
const INCLUDE_PATH: &str = "/home/skye/.local/include/azhdaha/";

/// Prevents predefining any system-specific or GCC-specific macros.
#[allow(dead_code)]
const UNDEF_FLAG: &str = "-undef";

/// Contains the source information about the code.
pub struct SourceInfo {
    /// Path to the target file.
    pub path: String,
    /// Preprocessed source code.
    pub code: Vec<u8>,
}

/// Replace headers with annotated versions and expand macros.
///
/// In order to only perform the preprocessing on the source code, [`PREPROCESS_ONLY_FLAG`]
/// is inserted into the arguments.
///
/// Headers used in the source code must be replaced with annotated versions which is accomplished by
/// inserting [`INCLUDE_FLAG`] pointing to a directory containing the annotated headers.
///
/// The "LINEAR_TYPE" macro used in the source code must be replaced by "linear_type" which is
/// accomplished by inserting [`INCLUDE_FLAG`] and redefining the macro.
///
pub(crate) fn preprocess(
    compile_commands: &CompilationDatabase,
) -> anyhow::Result<Vec<SourceInfo>> {
    let mut results = vec![];

    for compile_command in compile_commands {
        let Some(dir) = compile_command.directory.to_str() else {
            log::warn!("UTF-8 validity failed.");
            continue;
        };

        if !compile_command.directory.exists() {
            log::warn!("Directory '{dir}' does not exist.");
            continue;
        }

        let Some(args) = compile_command.arguments.as_ref() else {
            log::warn!("Arguments section was not found.");
            continue;
        };

        let (command, args, target) = match args {
            CompileArgs::Arguments(args) => {
                let (command, rest) = args.split_first().unwrap();
                let (target, args) = rest.split_last().unwrap();

                (command, args, target)
            }
            CompileArgs::Flags(_) => {
                // Arguments might be read from compile-flags which is not currently supported.
                panic!("Compile-flags is not currently supported.")
            }
        };

        results.push(SourceInfo {
            path: format!("{dir}/{target}"),
            code: Command::new(command)
                .arg(PREPROCESS_ONLY_FLAG)
                .arg(INHABIT_LINEMARKS_FLAG)
                .arg(INCLUDE_FLAG)
                .arg(INCLUDE_PATH)
                .args(args)
                .arg(target)
                .current_dir(dir)
                .output()?
                .stdout,
        });
    }

    Ok(results)
}
