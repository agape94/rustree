// use git2::Repository;

use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

mod clone;
mod worktree;

#[derive(Parser)]
#[command(name = "binary")]
#[command(
    about = "Helper tool that helps you cloning bare repositories and managing git worktrees for a bare repository"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone something
    Clone {
        /// Repository URL (can be SSH or HTTPS link)
        repository_url: String,

        /// Path to the SSH key.
        /// If not specified, all keys under '$HOME/.ssh' will be tried.
        #[arg(short, long)]
        ssh_key: Option<String>,
    },

    /// Manage worktrees
    Worktree {
        /// Path to worktree
        path: String,

        /// Branch for the new worktree. Cannot be a branch already used by another worktree.
        #[arg(long)]
        branch: String,

        /// Base branch for the new branch created for the worktree. If empty, the default branch of the repository will be used.
        #[arg(long)]
        base_branch: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clone {
            repository_url: repo,
            ssh_key,
        } => {
            let cwd = match env::current_dir() {
                Ok(cwd) => cwd,
                Err(_) => PathBuf::new(),
            };

            let ssh_key_path = if let Some(path) = ssh_key {
                PathBuf::from(path)
            } else {
                env::home_dir().unwrap().join(".ssh")
            };

            assert!(ssh_key_path.exists());

            let _repo = clone::clone_repository(repo, cwd, ssh_key_path)
                .expect("Could not clone repository.");
        }
        Commands::Worktree {
            path,
            branch,
            base_branch,
        } => {
            println!("Worktree path: {}", path);
            println!("Branch name: {}", branch);
            match base_branch {
                Some(value) => println!("Using base branch: {}", value),
                None => println!("No base branch provided"),
            };
        }
    }
}
