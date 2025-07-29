use crate::cli::HashType;
use blake2::{Blake2b512, Blake2s256, Digest};
use sha2::{Sha256, Sha512};
use std::fs;

pub fn compute_hash(file: &str, hash_type: &HashType) -> String {
    match hash_type {
        HashType::Md5 => compute_md5(file),
        HashType::Sha256 => compute_sha256(file),
        HashType::Sha512 => compute_sha512(file),
        HashType::Blake2s => compute_blake2s(file),
        HashType::Blake2b => compute_blake2b(file),
        HashType::Blake3 => compute_blake3(file),
    }
}

// compute_md5 computes the md5 hash of a given file
fn compute_md5(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = md5::compute(file_data);
    format!("{digest:x}")
}

// compute_sha256 computes the sha256 hash of a given file
fn compute_sha256(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Sha256::digest(file_data);
    format!("{digest:x}")
}

// compute_sha512 computes the sha512 hash of a given file
fn compute_sha512(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Sha512::digest(file_data);
    format!("{digest:x}")
}

// compute_blake2s computes the blake2s256 hash of a given file
fn compute_blake2s(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Blake2s256::digest(file_data);
    format!("{digest:x}")
}

// compute_blake2b computes the blake2b512 hash of a given file
fn compute_blake2b(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Blake2b512::digest(file_data);
    format!("{digest:x}")
}

// compute_blake3 computes the blake3 hash of a given file
fn compute_blake3(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let mut hasher = blake3::Hasher::new();
    hasher.update(file_data.as_slice());
    let digest = hasher.finalize();
    format!("{digest}")
}
