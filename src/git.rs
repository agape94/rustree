use crate::utils;
use git2::{Branch, Error, FetchOptions};
use git2::{Cred, RemoteCallbacks, Repository};
use ssh_key::PrivateKey;
use std::fs;
use std::path::PathBuf;

pub fn branch_exists_locally(repo: &Repository, name: &String) -> bool {
    repo.find_branch(name.as_str(), git2::BranchType::Local)
        .is_ok()
}

pub fn branch_exists_remotely(repo: &Repository, name: &String) -> bool {
    repo.find_branch(&format!("origin/{}", name), git2::BranchType::Remote)
        .is_ok()
}

fn create_branch_locally<'a>(
    repo: &'a Repository,
    name: &'a String,
    base_branch: &'a String,
) -> Result<Branch<'a>, Error> {
    let base_branch = repo
        .find_branch(&format!("origin/{}", base_branch), git2::BranchType::Remote)
        .unwrap();

    repo.branch(&name, &base_branch.get().peel_to_commit().unwrap(), false)
}

fn push_branch_to_remote(
    repo: &Repository,
    branch: &Branch,
    ssh_key: &PathBuf,
) -> Result<(), Error> {
    let mut remote = repo.find_remote("origin")?;

    let callbacks = get_credential_callbacks(&ssh_key);

    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let branch_name = branch.name().unwrap().unwrap().to_string();

    // local_ref:remote_ref
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

    remote.push(&[refspec], Some(&mut push_options))
}

fn set_upstream_for_branch(branch: &mut Branch) -> Result<(), Error> {
    let name = branch.name().unwrap().unwrap();
    branch.set_upstream(Some(&format!("origin/{}", name)))
}

pub fn create_branch(
    repo: &Repository,
    name: &String,
    base: Option<String>,
    ssh_key: &PathBuf,
) -> Result<(), Error> {
    let _callbacks = get_credential_callbacks(&ssh_key);

    let exists_locally = branch_exists_locally(&repo, &name);
    let exists_remotely = branch_exists_remotely(&repo, &name);

    if exists_locally && exists_remotely {
        // Branch already exists, we should not create it again, but we will push it to remote and set upstream
        println!("✅ Branch '{}' exists both locally and remotely", name);
        return Ok(());
    }

    // Doesn't exist either locally or remotely, we create it
    let base_branch = if let Some(b) = base {
        b
    } else {
        get_default_branch(&repo).unwrap()
    };

    if !exists_locally && !exists_remotely {
        // Create branch and push it to remote
        let mut branch = create_branch_locally(&repo, &name, &base_branch).unwrap();
        push_branch_to_remote(&repo, &branch, &ssh_key).unwrap();
        set_upstream_for_branch(&mut branch).unwrap();
        println!("✅ Created branch '{}' and pushed it to remote", name);
        return Ok(());
    } else if exists_locally && !exists_remotely {
        let mut branch = repo.find_branch(&name, git2::BranchType::Local).unwrap();
        push_branch_to_remote(&repo, &branch, &ssh_key).unwrap();
        set_upstream_for_branch(&mut branch).unwrap();
        println!("✅ Branch '{}' exists locally. Pushed it to remote", name);
        return Ok(());
    } else if !exists_locally && exists_remotely {
        let _remote_branch = repo
            .find_branch(&format!("origin/{}", name), git2::BranchType::Remote)
            .unwrap();
        let mut branch = create_branch_locally(&repo, &name, &base_branch).unwrap();
        set_upstream_for_branch(&mut branch).unwrap();
        println!(
            "✅ Created branch '{}' and set upstream to the existing branch on remote ",
            name
        );
        return Ok(());
    }
    return Err(Error::from_str(&format!(
        "❌ Could not create the branch '{}'",
        name
    )));
}

pub fn open_repository(path: PathBuf) -> Result<git2::Repository, git2::Error> {
    if path.exists() && path.is_dir() {
        Repository::open(path)
    } else {
        Err(git2::Error::from_str(&format!(
            "Could not open repository at path '{:?}'",
            path
        )))
    }
}

pub fn get_default_branch(repo: &Repository) -> Result<String, git2::Error> {
    let origin_head = repo.find_reference("refs/remotes/origin/HEAD").unwrap();

    let target = origin_head
        .symbolic_target()
        .ok_or_else(|| git2::Error::from_str("origin/HEAD not symbolic"))
        .unwrap();

    Ok(target
        .rsplit('/')
        .next()
        .ok_or_else(|| git2::Error::from_str("s"))
        .unwrap()
        .to_string())
}

pub fn get_ssh_keys(path: &PathBuf) -> Vec<PathBuf> {
    let mut ret: Vec<PathBuf> = Vec::new();
    if path.is_file() {
        let encoded_key = utils::read_file(&path).unwrap();

        PrivateKey::from_openssh(encoded_key).unwrap_or_else(|_| {
            panic!("❌ Could not identify a valid SSH key in {:?}", path);
        });

        ret.push(path.clone());
    } else if path.is_dir() {
        let files = fs::read_dir(&path).unwrap_or_else(|_| {
            panic!("❌ Directory '{:?}' is empty", path);
        });

        for file in files {
            if let Ok(_file) = file {
                let encoded_key = utils::read_file(&_file.path()).unwrap();
                if let Ok(_) = PrivateKey::from_openssh(encoded_key) {
                    ret.push(_file.path());
                }
            }
        }
        assert!(ret.len() > 0);
    }
    return ret;
}

pub fn get_credential_callbacks(ssh_key: &PathBuf) -> RemoteCallbacks<'_> {
    let ssh_keys = get_ssh_keys(&ssh_key);
    let mut key_iter = ssh_keys.into_iter();
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(move |_url, username_from_url, allowed_types| {
        let username =
            username_from_url.ok_or_else(|| git2::Error::from_str("No username provided"))?;

        // Only try ssh keys if ssh is allowed
        if allowed_types.is_ssh_key() {
            if let Some(key_path) = key_iter.next() {
                return Cred::ssh_key(username, None, &key_path, None);
            }
        }

        Err(git2::Error::from_str("No more SSH keys to try"))
    });

    return callbacks;
}

pub fn set_upstream_for_branches(repo: &mut Repository, ssh_key: &PathBuf) -> Result<(), String> {
    repo.remote_add_fetch("origin", "+refs/heads/*:refs/remotes/origin/*")
        .unwrap();

    let callbacks = get_credential_callbacks(&ssh_key);

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut origin = repo.find_remote("origin").unwrap();
    origin.fetch::<&str>(&[], Some(&mut fo), None).unwrap();

    let branches = repo.branches(Some(git2::BranchType::Local)).unwrap();
    for branch in branches {
        let (mut branch, _) = branch.unwrap();
        let branch_name = branch
            .name()
            .unwrap() // Result<Option<&str>, git2::Error>
            .expect("Branch has no name")
            .to_string();
        if let Ok(remote_branch) =
            repo.find_branch(&format!("origin/{}", branch_name), git2::BranchType::Remote)
        {
            let remote_name = remote_branch.name().unwrap();
            match branch.set_upstream(remote_name) {
                Ok(_) => {
                    continue;
                }
                Err(e) => panic!(
                    "Could not set upstream for branch {}. Error: {}",
                    &branch_name, e
                ),
            }
        }
    }

    Ok(())
}
