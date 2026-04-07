use crate::errors::ExtensionError;
use zed_extension_api::Worktree;

#[derive(Debug, Clone)]
pub struct WorkspacePaths {
    pub project_root: String,
    pub index_dir: String,
}

pub fn resolve_workspace_paths(
    worktree: Option<&Worktree>,
) -> Result<WorkspacePaths, ExtensionError> {
    let worktree = worktree.ok_or(ExtensionError::NoActiveWorktree)?;
    let project_root = worktree.root_path();
    let trimmed = project_root.trim_end_matches('/');
    let index_dir = format!("{trimmed}/.zed/srcsearch-index");

    Ok(WorkspacePaths {
        project_root,
        index_dir,
    })
}
