use crate::{git, worktree};
use git2::{Repository, build::RepoBuilder};
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::{fs, fs::File};

pub fn clone_repository(
    repo_url: String,
    repo_path: PathBuf,
    ssh_key: PathBuf,
) -> Result<Repository, String> {
    let repository_name: Vec<&str> = repo_url.split('/').collect();
    let repository_name: String = repository_name.last().unwrap().to_string();
    let repository_name: String = repository_name.split('.').next().unwrap().to_string();

    let mut repo_builder = RepoBuilder::new();
    repo_builder.bare(true);

    let repo_path = repo_path.join(repository_name);

    match fs::create_dir_all(&repo_path) {
        Ok(_) => println!("Created repository directory {:?}", repo_path),
        Err(_) => {
            println!("❌ Repository already exists! Exiting...");
            exit(-1);
        }
    };

    if !repo_url.contains("https") {
        // SSH repo URL
        // Find possible ssh keys in $HOME/.ssh/
        let callbacks = git::get_credential_callbacks(&ssh_key);
        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        repo_builder.fetch_options(fo);
    }

    let mut repo = repo_builder
        .clone(&repo_url, &repo_path.join(".bare"))
        .expect("❌ Could not clone repository");

    let mut file = match File::create(repo_path.join(".git")) {
        Ok(file) => file,
        Err(_) => exit(1),
    };

    file.write_all(b"gitdir: ./.bare\n")
        .expect("❌ Could not write to the .git file.");

    git::set_upstream_for_branches(&mut repo, &ssh_key).unwrap();
    let base_branch = git::get_default_branch(&repo).unwrap();

    std::env::set_current_dir(&repo_path).unwrap();

    worktree::create_worktree(
        &repo,
        base_branch.clone(),
        base_branch.clone(),
        base_branch.clone(),
        &ssh_key,
    )
    .unwrap();

    println!(
        "✅ Repository '{}' cloned successfully at '{}'",
        repo_url,
        repo_path.to_str().unwrap()
    );
    Ok(repo)
}
