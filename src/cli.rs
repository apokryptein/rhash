use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    author = "apokryptein",
    version,
    about = "simple hashing utility written in Rust"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Hash computes the hash(s) of the provided file(s)
    Hash {
        /// Hash algorithm to use
        #[arg(long, short = 't', value_enum)]
        hash_type: Option<HashType>,

        /// Files to hash
        files: Vec<String>,
    },
    /// Verify validates the file against a checksum file
    Verify {
        /// Checksum file to use during verification
        #[arg(long, short)]
        checksum_file: String,

        /// Hash algorithm used in checksum file
        #[arg(long, short = 't', value_enum)]
        hash_type: Option<HashType>,

        /// Files to verify
        files: Vec<String>,
    },
}

/// HashType is an enum containing available hashing functions
#[derive(Clone, ValueEnum, Debug)]
pub enum HashType {
    Md5,
    Sha256,
    Sha512,
    Blake2s,
    Blake2b,
    Blake3,
}
