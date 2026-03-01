use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub fn get_ssh_key_path(path: &Option<String>) -> PathBuf {
    let ssh_key_path = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        env::home_dir().unwrap().join(".ssh")
    };

    assert!(ssh_key_path.exists());
    return ssh_key_path;
}

pub fn read_file(path: &PathBuf) -> Result<String, String> {
    if path.is_file() {
        let mut file = fs::File::open(&path).unwrap_or_else(|_| {
            panic!("Could not open file at '{:?}'", path);
        });

        let mut encoded_key = String::new();
        fs::File::read_to_string(&mut file, &mut encoded_key).unwrap_or_else(|_| {
            panic!("❌ Could not read file from '{:?}'", path);
        });

        return Ok(encoded_key);
    }
    return Err(format!("❌ Could not open file at {:?}", path));
}
