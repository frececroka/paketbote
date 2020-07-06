use std::collections::HashMap;

use sha3::{Digest, Sha3_256};

pub mod home;
pub mod create_account;
pub mod login;
pub mod logout;
pub mod account;
pub mod access_tokens;
pub mod repo;
pub mod getfile;
pub mod upload;

fn no_context() -> HashMap<String, String> {
    HashMap::new()
}

fn hash_password(salt: &str, password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(salt);
    hasher.input(password);
    base64::encode(hasher.result())
}
