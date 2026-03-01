// use git2::Repository;

use clap::{Parser, Subcommand};
use std::env;

mod clone;
mod git;
mod utils;
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
        directory: String,

        /// Branch for the new worktree. Cannot be a branch already used by another worktree.
        branch: String,

        /// Base branch for the new branch created for the worktree. If empty, the default branch of the repository will be used.
        #[arg(long)]
        base_branch: Option<String>,

        /// Path to the SSH key.
        /// If not specified, all keys under '$HOME/.ssh' will be tried.
        #[arg(short, long)]
        ssh_key: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clone {
            repository_url: repo,
            ssh_key,
        } => {
            let cwd = env::current_dir().unwrap();
            let ssh_key_path = utils::get_ssh_key_path(&ssh_key);
            let _repo = clone::clone_repository(repo, cwd, ssh_key_path).unwrap();
        }

        Commands::Worktree {
            directory,
            branch,
            base_branch,
            ssh_key,
        } => {
            let repository_path = env::current_dir().unwrap();
            let mut repo = git::open_repository(repository_path).unwrap();

            let base_branch_str = if let Some(name) = base_branch {
                name
            } else {
                git::get_default_branch(&repo).unwrap().to_string()
            };

            let ssh_key_path = utils::get_ssh_key_path(&ssh_key);
            let _worktree = worktree::create_worktree(
                &mut repo,
                directory,
                branch,
                base_branch_str,
                &ssh_key_path,
            )
            .expect("Could not create worktree");
        }
    }
}
