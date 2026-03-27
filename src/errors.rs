use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ExtensionError {
    NoActiveWorktree,
    MissingBinary,
    EmptyQuery,
    MissingIndex,
    CommandSpawnFailed(String),
    CommandFailed {
        command: String,
        exit_code: Option<i32>,
        stderr: String,
    },
    EmptyStdout,
    InvalidUtf8(&'static str),
    InvalidJson(String),
    UnknownCommand(String),
}

impl Display for ExtensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoActiveWorktree => write!(
                f,
                "No active worktree found. Open a project folder in Zed and try again."
            ),
            Self::MissingBinary => write!(
                f,
                "srcsearch binary not found.\n\nInstall srcsearch and ensure it is available on PATH, then try again."
            ),
            Self::EmptyQuery => write!(f, "Please provide a search query."),
            Self::MissingIndex => write!(
                f,
                "No srcsearch index found for this workspace.\n\nRun /srcindex first."
            ),
            Self::CommandSpawnFailed(error) => write!(
                f,
                "Unable to run srcsearch.\n\nProcess error: {error}"
            ),
            Self::CommandFailed {
                command,
                exit_code,
                stderr,
            } => {
                let code_text = exit_code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                if stderr.trim().is_empty() {
                    write!(
                        f,
                        "srcsearch command failed: `{command}` (exit code: {code_text})."
                    )
                } else {
                    write!(
                        f,
                        "srcsearch command failed: `{command}` (exit code: {code_text}).\n\n{stderr}"
                    )
                }
            }
            Self::EmptyStdout => write!(f, "srcsearch returned no output."),
            Self::InvalidUtf8(stream) => write!(
                f,
                "srcsearch returned invalid UTF-8 on {stream}. Try updating srcsearch."
            ),
            Self::InvalidJson(error) => write!(
                f,
                "srcsearch returned invalid JSON output.\n\nDetails: {error}"
            ),
            Self::UnknownCommand(command) => write!(f, "unknown slash command: \"{command}\""),
        }
    }
}

impl ExtensionError {
    pub fn is_missing_index_signal(&self) -> bool {
        match self {
            Self::CommandFailed { stderr, .. } => {
                let lower = stderr.to_ascii_lowercase();
                lower.contains("no such file")
                    || lower.contains("not found")
                    || lower.contains("index") && lower.contains("exist")
            }
            _ => false,
        }
    }
}
