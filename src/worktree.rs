use git2::{Repository, WorktreeAddOptions};
use std::path::PathBuf;

use crate::git;

pub fn create_worktree(
    repo: &Repository,
    directory: String,
    branch_name: String,
    base_branch: String,
    ssh_key_path: &PathBuf,
) -> Result<PathBuf, String> {
    git::create_branch(repo, &branch_name, Some(base_branch), ssh_key_path).unwrap();

    let mut opts = WorktreeAddOptions::new();
    let reference = &repo
        .find_reference(&format!("refs/heads/{}", &branch_name))
        .unwrap();
    opts.reference(Some(reference));

    match repo.worktree(directory.as_str(), &PathBuf::from(&directory), Some(&opts)) {
        Ok(worktree) => {
            println!("✅ Worktree '{}' created successfuly", directory);
            Ok(PathBuf::from(worktree.path()))
        }
        Err(e) => {
            println!("❌ Could not create worktree '{}'", directory);
            Err(e.to_string())
        }
    }
}
