use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    author = "apokryptein",
    version,
    about = "simple hashing utility written in Rust"
)]
pub struct Args {
    #[arg(long, short = 't', value_enum)]
    pub hash_type: Option<HashType>,

    #[arg(long, short = 'c', conflicts_with = "hash_type")]
    pub check: Option<String>,

    pub files: Vec<String>,
}

#[derive(Clone, ValueEnum)]
pub enum HashType {
    Md5,
    Sha256,
    Sha512,
    Blake2s,
    Blake2b,
    Blake3,
}
