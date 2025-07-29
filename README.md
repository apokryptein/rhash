# rhash

A simple, fast command-line hashing tool written in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Features

- Hashing algorithms: MD5, SHA256, SHA512, BLAKE2s, BLAKE2b, BLAKE3
- Verify checksums from checksum files

## Installation

```bash
git clone https://github.com/apokryptein/rhash.git
cd rhash
cargo install --path <path>
```

## Usage

Computing hashes

```bash
# Defaults to SHA-256
rhash hash file.txt

# Using specific algorithms
rhash hash -t blake3 file.txt
rhash hash --hash-type sha512 *.txt

# Multiple files
rhash hash -t blake2s file1.txt file2.txt file3.txt
```

Verifying Checksums

- NOTE: verification functionality is still under development

```bash
# Verify all files in checksum file
rhash verify -c checksums.txt

# Verify specific files
rhash verify -c checksums.txt file1.txt file2.txt

# Specify hashing algorithm used in checksums file
rhash verify -c checksums.txt -t sha512
```
