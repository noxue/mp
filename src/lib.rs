pub mod msg;
pub mod reply;

use sha1::{Digest, Sha1};

pub fn check_signature(token: &str, signature: &str, timestamp: &str, nonce: &str) -> bool {
    let mut tmp_arr = vec![token, timestamp, nonce];
    tmp_arr.sort();
    let tmp_str = tmp_arr.join("");
    let mut hasher = Sha1::new();
    hasher.update(tmp_str.as_bytes());
    let tmp_str = format!("{:x}", hasher.finalize());
    tmp_str == signature
}
