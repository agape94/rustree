use clap::builder;
use core::error;
use git2::{Repository, build::RepoBuilder};
use std::alloc::System;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

pub fn clone_repository(repo_url: String, repo_path: PathBuf) {
    let repository_name: Vec<&str> = repo_url.split('/').collect();
    let repository_name = String::new(repository_name.last().unwrap());

    if repo_url.contains("https") {
        match fs::create_dir_all(&repo_path) {
            Ok(_) => println!("Created repository directory {:?}", repo_path.join("")),
            Err(_) => {
                println!("Repository already exists! Exiting...");
                exit(-1);
            }
        };

        let mut repo_builder = RepoBuilder::new();
        repo_builder.bare(true);

        let _repo = repo_builder
            .clone(&repo_url, &repo_path.join(".bare"))
            .expect("Could not clone repository...");
    }
}
