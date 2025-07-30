use crate::cli::HashType;
use anyhow::{Context, Result, anyhow, bail};
use blake2::{Blake2b512, Blake2s256, Digest};
use blake3::Hasher;
use sha2::{Sha256, Sha512};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// compute_hashes iterates over provided files and passes each file and hash_type to compute_hash
pub fn compute_hashes(files: &[String], hash_type: &HashType) -> Result<()> {
    for file in files {
        // Compute hash
        let hash = compute_hash(file, hash_type)?;

        // Print result
        println!("{hash}  {file}");
    }

    Ok(())
}

/// compute_hash checks for algorithm and passes to appropriate hashing function
fn compute_hash(file: &str, hash_type: &HashType) -> Result<String> {
    match hash_type {
        HashType::Md5 => compute_md5(file),
        HashType::Sha256 => compute_sha256(file),
        HashType::Sha512 => compute_sha512(file),
        HashType::Blake2s => compute_blake2s(file),
        HashType::Blake2b => compute_blake2b(file),
        HashType::Blake3 => compute_blake3(file),
    }
}

/// verify_checksums
pub fn verify_checksums(
    checksum_file: &str,
    files: &[String],
    hash_type: HashType,
    bsd: bool,
) -> Result<()> {
    // BSD formatting no yet supported
    if bsd {
        return Err(anyhow!(
            "[ERR] BSD formatted checksum verification not yet implemented"
        ));
    }

    // Open handle to checksum_file
    let check_file = File::open(checksum_file)
        .with_context(|| anyhow!("[ERR] failed to open checksum file: {checksum_file}"))?;

    // Get path for checksum_file
    let checksum_dir = Path::new(checksum_file).parent().unwrap_or(Path::new("."));

    // Instantiate new reader
    let reader = BufReader::new(check_file);

    // Track if a failure has occurred
    let mut had_failure = false;

    // Track found and verified files
    let mut verified_files: HashSet<String> = std::collections::HashSet::new();

    // Read lines from checksum file
    for line in reader.lines() {
        // Get line
        let line = line?;

        // Skip empty and commented lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split line into hash and filename
        // Expected format:
        // ed3644332d10d8804d58f64e9fd9471bdfa71f60f2c2704f5c9d70531b070f69  debian-live-12.11.0-amd64-xfce.iso
        let line_parts: Vec<&str> = line.splitn(2, "  ").collect();

        // Check number of parts
        if line_parts.len() != 2 {
            // Just show error and continue - no need to throw the baby out
            // with the bath water
            eprintln!("[ERR] invalid line format: {line}");
            continue;
        }

        // Grab parts
        let (expected_hash, filename) = (line_parts[0], line_parts[1]);

        // If filenames were provided, verify
        if !files.is_empty() {
            // Ensure filename is in provided files to verify
            let matching_file = files
                .iter()
                .find(|&file| Path::new(file).file_name() == Path::new(&filename).file_name());

            if let Some(file_to_verify) = matching_file {
                // Add file to verified
                verified_files.insert(file_to_verify.clone());

                match verify_file(file_to_verify, expected_hash, &hash_type) {
                    Ok(true) => println!("{filename}:  OK"),
                    Ok(false) => {
                        println!("{filename}: FAILED");
                        had_failure = true;
                    }
                    Err(e) => {
                        println!("[ERR] failure on file: {filename}");
                        eprintln!("    {e:#}");
                        had_failure = true;
                    }
                }
            }
        } else {
            // If we arrive here, it is assumed that the user wants to check
            // all files in the checksum

            // Construct full path
            let file_path = checksum_dir.join(filename);
            let file_path_str = file_path.to_string_lossy();

            match verify_file(&file_path_str, expected_hash, &hash_type) {
                Ok(true) => println!("{filename}:  OK"),
                Ok(false) => {
                    println!("{filename}: FAILED");
                    had_failure = true;
                }
                Err(e) => {
                    // Check for a file not found error
                    if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                        if io_err.kind() == std::io::ErrorKind::NotFound {
                            println!("{filename}: NOT FOUND");
                            had_failure = true;
                            continue;
                        }
                    }

                    // Other errors
                    println!("{filename}: FAILED open or read");
                    eprintln!("  {e:#}");
                    had_failure = true;
                }
            }
        }
    }

    // Report back any files not found in checksum
    if !files.is_empty() {
        for file in files {
            if !verified_files.contains(file) {
                eprintln!(
                    "{}: NOT FOUND in checksum file",
                    Path::new(file)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(file)
                );
                had_failure = true;
            }
        }
    }

    if had_failure {
        bail!("[FAIL] Checksum verification failed");
    }

    Ok(())
}

/// verify_file verifies a file's provided hash against a reculculation
fn verify_file(filename: &str, expected_hash: &str, hash_type: &HashType) -> Result<bool> {
    // Compute hash of file
    let computed = compute_hash(filename, hash_type)?;

    // Compare and return match case
    Ok(computed.eq_ignore_ascii_case(expected_hash))
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
