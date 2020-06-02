# Chapter 3

You're finally ready to implement a full PNG file. It's very complicated.

First you need a header containing 8 bytes that are always the same. Then you need a list of chunks.

Ok maybe it's not that complicated.

As usual, you can refer to the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
to make sure you're doing it right. If you're tired of reading that page, this is the last time you'll have to do it.

When you're done with this chapter, you'll have a nice little library that does everything you need to implement your command line program. This is also the last chapter with unit tests. It's all downhill from here!


## Assignment
Using the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html), implement PNG files.

You need to provide a constructor that takes a list of chunks, methods to append and remove chunks, and methods that return the header, a slice of all chunks, and the entire PNG file as a `Vec<u8>` of bytes.


## Requirements
1. Copy the unit tests at the bottom of this page and paste them at the bottom of your `png.rs` file.
2. Write a `Png` struct with your implementation of PNG files.
3. In your `impl` block, add a public constant called `STANDARD_HEADER` that has the 8 standard header bytes.
4. Implement `TryFrom<&[u8]>` for your `Png`.
5. Implement `Display` for your `Png`.
6. Required methods:
   1. `fn from_chunks(chunks: Vec<Chunk>) -> Png`
   2. `fn append_chunk(&mut self, chunk: Chunk)`
   3. `fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk>`
   4. `fn header(&self) -> &[u8; 8]`
   5. `fn chunks(&self) -> &[Chunk]`
   6. `fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk>`
   7. `fn as_bytes(&self) -> Vec<u8>`
7. Pass all of the unit tests.


## Unit Tests

```rust
{{#include tests/png_tests.rs}}
```

## Resources
* [PNG Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)
* [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
* [std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
* [std::convert::AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html) 
* [std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html)
* [std::fs::read](https://doc.rust-lang.org/std/fs/fn.read.html)
* [Iterator::enumerate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)
* [std::iter::Extend](https://doc.rust-lang.org/std/iter/trait.Extend.html)
* [std::io::Read](https://doc.rust-lang.org/std/io/trait.Read.html)
* [Read::read_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact)
* [std::io::BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
* [u32::from_be_bytes](https://doc.rust-lang.org/std/primitive.u32.html#method.from_be_bytes)
