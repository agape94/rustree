use git2::{Repository, build::RepoBuilder};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

pub fn clone_repository(repo_url: String, repo_path: PathBuf) -> Result<Repository, String> {
    let repository_name: Vec<&str> = repo_url.split('/').collect();
    let repository_name: String = repository_name.last().unwrap().to_string();
    let repository_name: String = repository_name.split('.').next().unwrap().to_string();

    let repo_path = repo_path.join(repository_name);

    if repo_url.contains("https") {
        match fs::create_dir_all(&repo_path) {
            Ok(_) => println!("Created repository directory {:?}", repo_path),
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

        let mut file = match File::create(repo_path.join(".git"))
        {
            Ok(file) => file,
            Err(_) => exit(1),
        };

        file.write_all(b"gitdir: ./.bare\n").expect("Could not write to the .git file.");
        Ok(_repo)

    }
    else {
        return Err("SSH not supported yet.".to_string());
    }
}