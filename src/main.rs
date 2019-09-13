extern crate crypto;
//extern crate systemstat;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
//use systemstat::{ System, Platform };

use std::io::{ self, Read };
use std::fs::{ self, File };
use std::path::Path;

fn main() {
    let mounts = if cfg!(target_family = "windows") { // Windows doesn't have /
        /*
        let sys = System::new();
        match sys.mounts() {
            Ok(mounts) => mounts.iter().map(|mount| mount.fs_mounted_on.clone()).collect::<Vec<String>>(),
            Err(_) => {
                println!("Error getting device drives. SYSTEM IS NOT SCANNED!");
                pause();
                return;
            },
        }
        */
        vec![String::from("C:/")]
    } else {
        vec![String::from("/")]
    };

    let hashes_string = env!("FILE_SCANNER_HASHES");
    let hashes = hashes_string.split(',').map(|hash| hash.trim().to_lowercase()).collect::<Vec<String>>();

    for mount in mounts {
        let path = Path::new(&mount);
        visit_dirs(&path, &hashes);
    }

    pause();
}

// Recursive function
fn visit_dirs(dir: &Path, hashes:&[String]) {
    if dir.is_dir() {
        let dir_iter = match fs::read_dir(dir) {
            Ok(dir_iter) => dir_iter,
            Err(_) => return,
        };
        for entry in dir_iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, hashes);
            } else {
                let mut file = match File::open(&path) {
                    Ok(file) => file,
                    Err(_) => {
                        println!("Error reading file {}", path.display());
                        return;
                    },
                };
                let mut file_bytes = Vec::new();
                match file.read_to_end(&mut file_bytes) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error reading file {}", path.display());
                        return;
                    },
                };

                // Check MD5
                let mut hasher = Md5::new();
                hasher.input(&file_bytes);
                let hex = hasher.result_str();
                if hashes.iter().any(|hash| *hash == hex) {
                    println!("Match for {} detected at {}", hex, path.display());
                    continue;
                }

                // Check SHA1
                let mut hasher = Sha1::new();
                hasher.input(&file_bytes);
                let hex = hasher.result_str();
                if hashes.iter().any(|hash| *hash == hex) {
                    println!("Match for {} detected at {}", hex, path.display());
                    continue;
                }

                // Check SHA256
                let mut hasher = Sha256::new();
                hasher.input(&file_bytes);
                let hex = hasher.result_str();
                if hashes.iter().any(|hash| *hash == hex) {
                    println!("Match for {} detected at {}", hex, path.display());
                    continue;
                }
            }
        }
    }
}

fn pause() {
    println!("Please press Enter to continue...");
    let _ = io::stdin().read(&mut [0]);
}
