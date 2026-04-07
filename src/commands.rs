use crate::{
    errors::ExtensionError,
    parse::parse_search_hits,
    process::run_srcsearch,
    render::{render_index_success, render_search_results, render_update_success},
    worktree::resolve_workspace_paths,
};
use zed_extension_api::{SlashCommandOutput, SlashCommandOutputSection, Worktree};

pub fn run(
    command_name: &str,
    args: Vec<String>,
    worktree: Option<&Worktree>,
) -> Result<SlashCommandOutput, String> {
    let result = match command_name {
        "srcindex" => run_index(worktree),
        "srcupdate" => run_update(worktree),
        "srcsearch" => run_search(worktree, args, false),
        "srcdocs" => run_search(worktree, args, true),
        other => Err(ExtensionError::UnknownCommand(other.to_string())),
    }
    .map_err(|error| error.to_string())?;

    Ok(output_with_single_section("srcsearch", result))
}

fn run_index(worktree: Option<&Worktree>) -> Result<String, ExtensionError> {
    let worktree = worktree.ok_or(ExtensionError::NoActiveWorktree)?;
    let paths = resolve_workspace_paths(Some(worktree))?;

    let args = vec![
        "index".to_string(),
        "--project-root".to_string(),
        paths.project_root.clone(),
        "--output-dir".to_string(),
        paths.index_dir.clone(),
    ];

    run_srcsearch(worktree, &args, false)?;

    Ok(render_index_success(&paths.project_root, &paths.index_dir))
}

fn run_update(worktree: Option<&Worktree>) -> Result<String, ExtensionError> {
    let worktree = worktree.ok_or(ExtensionError::NoActiveWorktree)?;
    let paths = resolve_workspace_paths(Some(worktree))?;

    // Phase 1 fallback: a refresh is implemented as an explicit rebuild.
    let args = vec![
        "index".to_string(),
        "--project-root".to_string(),
        paths.project_root.clone(),
        "--output-dir".to_string(),
        paths.index_dir.clone(),
    ];

    run_srcsearch(worktree, &args, false)?;

    Ok(render_update_success(&paths.project_root, &paths.index_dir))
}

fn run_search(
    worktree: Option<&Worktree>,
    args: Vec<String>,
    docs_only: bool,
) -> Result<String, ExtensionError> {
    let query = args.join(" ").trim().to_string();
    if query.is_empty() {
        return Err(ExtensionError::EmptyQuery);
    }

    let worktree = worktree.ok_or(ExtensionError::NoActiveWorktree)?;
    let paths = resolve_workspace_paths(Some(worktree))?;

    let mut command_args = vec![
        "search".to_string(),
        "--index-dir".to_string(),
        paths.index_dir,
        "--query".to_string(),
        query.clone(),
        "--json".to_string(),
    ];

    if docs_only {
        command_args.push("--scope".to_string());
        command_args.push("doc".to_string());
    }

    let process_output = match run_srcsearch(worktree, &command_args, true) {
        Ok(output) => output,
        Err(error) if error.is_missing_index_signal() => return Err(ExtensionError::MissingIndex),
        Err(error) => return Err(error),
    };

    let hits = parse_search_hits(&process_output.stdout)?;
    Ok(render_search_results(&query, docs_only, &hits))
}

fn output_with_single_section(label: &str, text: String) -> SlashCommandOutput {
    SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label: label.to_string(),
        }],
        text,
    }
}
