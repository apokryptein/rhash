use clap::Parser;
use cli::{Args, HashType};

mod cli;
mod hash;

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
    println!("{checksum_file} {files:?}");
}

// handler to check HashType and route to appropriate hashing function
fn compute_hashes(hash_type: HashType, files: &[String]) {
    for file in files {
        let hash = hash::compute_hash(file, &hash_type);
        println!("{hash}  {file}");
    }
}
