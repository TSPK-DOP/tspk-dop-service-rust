use sha256::digest;

pub fn hash_password(password: &str) -> String {
    digest(password)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    digest(password) == hash
}
