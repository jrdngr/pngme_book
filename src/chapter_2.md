# Chapter 2 - Chunks

Now that we've got our `ChunkType` struct, we can implement the rest of the our chunks. You'll be using the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) again. Section `3.2` has all of the information you need.

The toughest part of this assignment will be creating a chunk from a sequence of bytes using the `TryFrom` trait. Implementing the `as_bytes()` method may also be tricky depending on how much experience you have working with iterators. Check the hints for this chapter if you get stuck.


### Calculating the CRC
Check out [crc::crc32::checksum_ieee](https://docs.rs/crc/1.8.1/crc/crc32/fn.checksum_ieee.html) in the [crc](https://crates.io/crates/crc) crate. If you really want to implement this yourself, have at it. Your CRC needs to match the CRC that I get from the `crc` crate.

Don't forget to include the chunk type in your CRC calculation.


## Assignment

Using the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html), implement PNG chunks.
Your chunks should contain the four pieces of data laid out in section `3.2` of this link.

You need to provide methods that return each of the four pieces of data, the chunk data interpreted as a string if it is valid UTF-8 (or an error otherwise), and the entire chunk as a sequence of bytes in the order required by the PNG spec. Method signatures are provided below.


## Requirements
1. Copy the unit tests at the bottom of this page and paste them at the bottom of your `chunk.rs` file.
2. Write a `Chunk` struct with your implementation of PNG chunks.
3. Implement `TryFrom<&[u8]>` for your `Chunk`.
4. Implement `Display` for your `ChunkType`.
5. Required methods:
   1. `fn length(&self) -> u32`
   2. `fn chunk_type(&self) -> &ChunkType`
   3. `fn data(&self) -> &[u8]`
   4. `fn crc(&self) -> u32`
   5. `fn data_as_string(&self) -> Result<String>`
   6. `fn as_bytes(&self) -> Vec<u8>`
6. Pass all of the unit tests.


## Unit Tests

```rust
{{#include tests/chunk_tests.rs}}
```


## Resources
* [PNG Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)
* [crc](https://github.com/mrhooray/crc-rs)
* [crc::crc32::checksum_ieee](https://docs.rs/crc/1.8.1/crc/crc32/fn.checksum_ieee.html)
* [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
* [std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
* [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
* [String::from_utf8](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8)
* [str::bytes](https://doc.rust-lang.org/std/primitive.str.html#method.bytes)
* [str::as_bytes](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes)
* [Iterator::chain](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)
* [Iterator::collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
* [Iterator::cloned](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)
* [Iterator::copied](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied)
* [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
* [std::io::Read](https://doc.rust-lang.org/std/io/trait.Read.html)
* [Read::read_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact)
* [std::io::BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
* [u32::from_be_bytes](https://doc.rust-lang.org/std/primitive.u32.html#method.from_be_bytes)

