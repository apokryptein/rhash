use blake2::{Blake2b512, Blake2s256, Digest};
use clap::{Parser, ValueEnum};
use sha2::{Sha256, Sha512};
use std::fs;

#[derive(Parser)]
#[command(
    author = "apokryptein",
    version,
    about = "simple hashing utility written in Rust"
)]
struct Args {
    #[arg(long, short = 't', value_enum)]
    hash_type: Option<HashType>,

    #[arg(long, short = 'c', conflicts_with = "hash_type")]
    check: Option<String>,

    files: Vec<String>,
}

#[derive(Clone, ValueEnum)]
enum HashType {
    Md5,
    Sha256,
    Sha512,
    Blake2s,
    Blake2b,
    Blake3,
}

fn main() {
    let args = Args::parse();

    if let Some(checksum_file) = &args.check {
        verify_checksums(checksum_file, &args.files);
    } else if let Some(hash_type) = args.hash_type {
        compute_hashes(hash_type, &args.files);
    } else {
        eprintln!("Specify either --hash-type <TYPE> or --check <FILE>");
    }
}

fn verify_checksums(checksum_file: &str, files: &[String]) {
    // TODO: implement this
    println!("{} {:?}", checksum_file, files);
}

// handler to check HashType and route to appropriate hashing function
fn compute_hashes(hash_type: HashType, files: &[String]) {
    for file in files {
        let hash = match hash_type {
            HashType::Md5 => compute_md5(file),
            HashType::Sha256 => compute_sha256(file),
            HashType::Sha512 => compute_sha512(file),
            HashType::Blake2s => compute_blake2s(file),
            HashType::Blake2b => compute_blake2b(file),
            HashType::Blake3 => compute_blake3(file),
        };
        println!("{}  {}", hash, file);
    }
}

// compute_md5 computes the md5 hash of a given file
fn compute_md5(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = md5::compute(file_data);
    format!("{:x}", digest)
}

// compute_sha256 computes the sha256 hash of a given file
fn compute_sha256(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Sha256::digest(file_data);
    format!("{:x}", digest)
}

// compute_sha512 computes the sha512 hash of a given file
fn compute_sha512(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Sha512::digest(file_data);
    format!("{:x}", digest)
}

// compute_blake2s computes the blake2s256 hash of a given file
fn compute_blake2s(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Blake2s256::digest(file_data);
    format!("{:x}", digest)
}

// compute_blake2b computes the blake2b512 hash of a given file
fn compute_blake2b(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let digest = Blake2b512::digest(file_data);
    format!("{:x}", digest)
}

// compute_blake3 computes the blake3 hash of a given file
fn compute_blake3(file: &str) -> String {
    let file_data = fs::read(file).expect("Failed to read file");
    let mut hasher = blake3::Hasher::new();
    hasher.update(file_data.as_slice());
    let digest = hasher.finalize();
    format!("{}", digest)
}
