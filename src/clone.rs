use git2::{Cred, RemoteCallbacks, Repository, build::RepoBuilder};
use ssh_key::PrivateKey;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{fs, fs::File};

fn read_file(path: &PathBuf) -> Result<String, String> {
    if path.is_file() {
        let mut file = fs::File::open(&path).unwrap_or_else(|_| {
            panic!("Could not open file at '{:?}'", path);
        });

        let mut encoded_key = String::new();
        fs::File::read_to_string(&mut file, &mut encoded_key).unwrap_or_else(|_| {
            panic!("Could not read file from '{:?}'", path);
        });

        return Ok(encoded_key);
    }
    return Err(format!("Could not open file at {:?}", path));
}

fn get_ssh_keys(path: PathBuf) -> Vec<PathBuf> {
    let mut ret: Vec<PathBuf> = Vec::new();
    if path.is_file() {
        let encoded_key = read_file(&path).unwrap();

        PrivateKey::from_openssh(encoded_key).unwrap_or_else(|_| {
            panic!("Could not identify a valid SSH key in {:?}", path);
        });

        ret.push(path);
    } else if path.is_dir() {
        let files = fs::read_dir(&path).unwrap_or_else(|_| {
            panic!("Directory '{:?}' is empty", path);
        });

        for file in files {
            if let Ok(_file) = file {
                println!("{:?}", _file.path());
                let encoded_key = read_file(&_file.path()).unwrap();
                if let Ok(_) = PrivateKey::from_openssh(encoded_key) {
                    ret.push(_file.path());
                }
            }
        }
        assert!(ret.len() > 0);
    }
    return ret;
}

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
            println!("Repository already exists! Exiting...");
            exit(-1);
        }
    };

    if !repo_url.contains("https") {
        // SSH repo URL
        // Find possible ssh keys in $HOME/.ssh/
        let ssh_keys = get_ssh_keys(ssh_key);
        let mut key_iter = ssh_keys.into_iter();
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(move |_url, username_from_url, allowed_types| {
            let username =
                username_from_url.ok_or_else(|| git2::Error::from_str("No username provided"))?;

            // Only try ssh keys if ssh is allowed
            if allowed_types.is_ssh_key() {
                if let Some(key_path) = key_iter.next() {
                    println!("Trying SSH key: {:?}", key_path);
                    return Cred::ssh_key(username, None, &key_path, None);
                }
            }

            Err(git2::Error::from_str("No more SSH keys to try"))
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

    let mut file = match File::create(repo_path.join(".git")) {
        Ok(file) => file,
        Err(_) => exit(1),
    };

    file.write_all(b"gitdir: ./.bare\n")
        .expect("Could not write to the .git file.");
    println!(
        "✅ Repository '{}' cloned successfully at '{}'",
        repo_url,
        repo_path.to_str().unwrap()
    );
    Ok(_repo)
}
