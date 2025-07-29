use crate::cli::HashType;
use anyhow::{Context, Result, anyhow};
use blake2::{Blake2b512, Blake2s256, Digest};
use blake3::Hasher;
use sha2::{Sha256, Sha512};
use std::fs;

/// compute_hash checks for algorithm and passes to appropriate hashing function
pub fn compute_hashes(files: &[String], hash_type: &HashType) -> Result<()> {
    for file in files {
        let hash = match hash_type {
            HashType::Md5 => compute_md5(file),
            HashType::Sha256 => compute_sha256(file),
            HashType::Sha512 => compute_sha512(file),
            HashType::Blake2s => compute_blake2s(file),
            HashType::Blake2b => compute_blake2b(file),
            HashType::Blake3 => compute_blake3(file),
        }?;

        println!("{hash}  {file}");
    }

    Ok(())
}

/// verify_checksums
pub fn verify_checksums(
    checksum_file: &str,
    files: &[String],
    hash_type: Option<HashType>,
    bsd: bool,
) -> Result<()> {
    // TODO: implement this
    // 1. read file from disk
    // 2. iterate over files
    // 3. skip comments (#) and empty lines
    // 4. split each line into vec & verify number of parts (2)
    // 5. pass to another verification function that calls computes_hashes
    // and returns a book if there is a match
    println!("{checksum_file} {files:?} {hash_type:?} {bsd}");

    Err(anyhow!("[ERR] verify_checksums yet to be implemented"))
}

/// compute_md5 computes the md5 hash of a given file
fn compute_md5(file: &str) -> Result<String> {
    let file_data = fs::read(file).with_context(|| anyhow!("[ERR] failed to open file: {file}"))?;
    let digest = md5::compute(file_data);

    Ok(format!("{digest:x}"))
}

/// compute_sha256 computes the sha256 hash of a given file
fn compute_sha256(file: &str) -> Result<String> {
    let file_data = fs::read(file).with_context(|| anyhow!("[ERR] failed to open file: {file}"))?;
    let digest = Sha256::digest(file_data);

    Ok(format!("{digest:x}"))
}

/// compute_sha512 computes the sha512 hash of a given file
fn compute_sha512(file: &str) -> Result<String> {
    let file_data = fs::read(file).with_context(|| anyhow!("[ERR] failed to open file: {file}"))?;
    let digest = Sha512::digest(file_data);

    Ok(format!("{digest:x}"))
}

/// compute_blake2s computes the blake2s256 hash of a given file
fn compute_blake2s(file: &str) -> Result<String> {
    let file_data = fs::read(file).with_context(|| anyhow!("[ERR] failed to open file: {file}"))?;
    let digest = Blake2s256::digest(file_data);

    Ok(format!("{digest:x}"))
}

/// compute_blake2b computes the blake2b512 hash of a given file
fn compute_blake2b(file: &str) -> Result<String> {
    let file_data = fs::read(file).with_context(|| anyhow!("[ERR] failed to open file: {file}"))?;
    let digest = Blake2b512::digest(file_data);

    Ok(format!("{digest:x}"))
}

/// compute_blake3 computes the blake3 hash of a given file
fn compute_blake3(file: &str) -> Result<String> {
    let file_data = fs::read(file).with_context(|| anyhow!("[ERR] failed to open file: {file}"))?;
    let mut hasher = Hasher::new();
    hasher.update(file_data.as_slice());
    let digest = hasher.finalize();

    Ok(format!("{digest}"))
}
