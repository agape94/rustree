// use git2::Repository;

use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

mod clone;
mod worktree;

#[derive(Parser)]
#[command(name = "binary")]
#[command(about = "Example CLI with subcommands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone something
    Clone {
        /// Repository URL (can be SSH or HTTPS link)
        repo: String,
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
        Commands::Clone { repo } => {
            let cwd = match env::current_dir() {
                Ok(cwd) => cwd,
                Err(_) => PathBuf::new(),
            };
            let _repo = clone::clone_repository(repo, cwd).expect("Could not clone repository.");
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
