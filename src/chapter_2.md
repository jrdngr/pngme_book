# Chapter 2

## Assignment
Implement PNG chunks.

`BufReader::read_exact`


## Calculating the CRC
Check out [crc::crc32::checksum_ieee](https://docs.rs/crc/1.8.1/crc/crc32/fn.checksum_ieee.html) in the [crc](https://crates.io/crates/crc) crate.


`u32::from_be_bytes`


```rust
{{#include tests/chunk_tests.rs}}
```
