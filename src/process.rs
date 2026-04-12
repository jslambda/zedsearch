use crate::errors::ExtensionError;
use zed_extension_api::{process::Command, Worktree};

#[derive(Debug, Clone)]
pub struct ProcessOutput {
    pub stdout: String,
}

pub fn run_srcsearch(
    worktree: &Worktree,
    args: &[String],
    expect_stdout: bool,
) -> Result<ProcessOutput, ExtensionError> {
    let binary = worktree
        .which("srcsearch")
        .ok_or(ExtensionError::MissingBinary)?;

    let mut command = Command::new(binary.clone())
        .args(args.iter().cloned())
        .envs(worktree.shell_env());

    let output = command
        .output()
        .map_err(ExtensionError::CommandSpawnFailed)?;

    let stdout =
        String::from_utf8(output.stdout).map_err(|_| ExtensionError::InvalidUtf8("stdout"))?;
    let stderr =
        String::from_utf8(output.stderr).map_err(|_| ExtensionError::InvalidUtf8("stderr"))?;

    if output.status != Some(0) {
        let command_text = format!("{binary} {}", args.join(" "));
        return Err(ExtensionError::CommandFailed {
            command: command_text,
            exit_code: output.status,
            stderr,
        });
    }

    if expect_stdout && stdout.trim().is_empty() {
        return Err(ExtensionError::EmptyStdout);
    }

    Ok(ProcessOutput { stdout })
}
