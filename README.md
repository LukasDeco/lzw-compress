# LZW Compression Library

![Rust Version](https://img.shields.io/badge/rust-1.57.0+-blue.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A Rust library for LZW data compression and decompression.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
  - [Installation](#installation)
  - [Compression](#compression)
  - [Decompression](#decompression)
- [Examples](#examples)
- [License](#license)

## Introduction

This Rust library provides a simple and efficient implementation of the LZW data compression algorithm. LZW is a widely used compression algorithm that can be used to reduce the size of data for storage or transmission.

## Features

- Compression of data into LZW-encoded format.
- Decompression of LZW-encoded data to its original format.
- Easy-to-use Rust API.

## Usage

### Installation

To use this library in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
lzw-compression = "0.1.0"
```

Or call `cargo add lzw-compression`

### Compression

To compress data using the LZW algorithm, you can use the `compress` function provided by the library. Here's an example of how to use it:

```rust
use lzw_compression::compress;

let data = vec![1, 2, 3, 4, 1, 2, 3, 5];
let compressed_data = compress(&data);
println!("Compressed data: {:?}", compressed_data);

use lzw_compression::decompress;

let compressed_data = vec![1, 2, 3, 4, 1, 2, 3, 5];
let decompressed_data = decompress(&compressed_data);
println!("Decompressed data: {:?}", decompressed_data);
```

## License

This library is licensed under the [MIT License](LICENSE).
