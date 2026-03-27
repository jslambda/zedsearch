mod commands;
mod errors;
mod parse;
mod process;
mod render;
mod worktree;

use zed_extension_api::{self as zed, SlashCommand, SlashCommandOutput, Worktree};

struct SrcsearchExtension;

impl zed::Extension for SrcsearchExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        commands::run(&command.name, args, worktree)
    }
}

zed::register_extension!(SrcsearchExtension);
