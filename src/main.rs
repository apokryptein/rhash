use anyhow::Result;
use clap::Parser;
use cli::{Args, HashType};

mod cli;
mod hash;

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    // Match on provided command and pass to appropriate function
    match args.command {
        cli::Command::Hash { hash_type, files } => {
            let hash_type = hash_type.unwrap_or(HashType::Sha256);
            hash::compute_hashes(&files, &hash_type)?
        }
        cli::Command::Verify {
            checksum_file,
            hash_type,
            files,
        } => hash::verify_checksums(&checksum_file, &files, hash_type)?,
    }

    Ok(())
}
