// Written in different file so soroban_sdk::Vec not colliding with std::vec::Vec
use soroban_sdk::{Bytes, Env, Vec};

pub fn str_to_bytes(env: &Env, string: &str) -> Bytes {
    let src: &[u8] = string.as_bytes();
    let bytes: Bytes = Bytes::from_slice(&env, src);
    return bytes;
}

pub fn str_arr_to_bytes(env: &Env, string: &[&str]) -> Vec<Bytes> {
    let arr_length = string.len();
    let mut bytes_vec: Vec<Bytes> = Vec::new(&env);
    for i in 0..arr_length {
        let src: &[u8] = string[i].as_bytes();
        let bytes: Bytes = Bytes::from_slice(&env, src);
        bytes_vec.push_back(bytes);
    }
    return bytes_vec;
}
