use git2::{Cred, Repository, build::RepoBuilder, RemoteCallbacks};
use std::{fs, fs::File, env};
use std::io::Write;
use std::path::{PathBuf, Path};
use std::process::exit;

pub fn clone_repository(repo_url: String, repo_path: PathBuf) -> Result<Repository, String> {
    let repository_name: Vec<&str> = repo_url.split('/').collect();
    let repository_name: String = repository_name.last().unwrap().to_string();
    let repository_name: String = repository_name.split('.').next().unwrap().to_string();
    
    let mut repo_builder = RepoBuilder::new();
    repo_builder.bare(true);

    let repo_path = repo_path.join(repository_name);
    
    match fs::create_dir_all(&repo_path) {
        Ok(_) => println!("Created repository directory {:?}", repo_path),
        Err(_) => {
            println!("Repository already exists! Exiting...");
            exit(-1);
        }
    };

    if !repo_url.contains("https") {
        // SSH repo URL
        // Prepare callbacks.
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
            None,
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        repo_builder.fetch_options(fo);
    }

    let _repo = repo_builder
        .clone(&repo_url, &repo_path.join(".bare"))
        .expect("Could not clone repository...");

    let mut file = match File::create(repo_path.join(".git"))
    {
        Ok(file) => file,
        Err(_) => exit(1),
    };

    file.write_all(b"gitdir: ./.bare\n").expect("Could not write to the .git file.");
    println!("✅ Repository '{}' cloned successfully at '{}'", repo_url, repo_path.to_str().unwrap());
    Ok(_repo)

}