# Chapter 1 - Chunk Types

Okay here we go!

The next three chapters will have you implement a basic PNG file. PNG files are essentially just a list of "chunks", each containing their own data. Each chunk has a type that can be represented as a 4 character string. There are standard chunk types for things like image data, but there's no rule that would prevent you from inserting your own chunks with whatever data you want. We can even tell PNG decoders to ignore our chunks depending on how we capitalize our chunk types.

In this chapter, we'll be implementing chunk types. These are pretty easy since they're essentially just 4 alphabetic characters. Your chunk types should always be __valid__ chunks. It should not be possible to construct an invalid chunk type using your public interface. The [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) will tell what a valid chunk type looks like.

### A little bit about bytes

I keep saying that chunk types are strings, but you'll want to think about them in terms of bytes. If you've never worked directly with bytes before, it's not that scary. If you _have_ worked directly with bytes before, you can skip to the [Assignment](#assignment) section below.

A bit is a `0` or a `1`. A byte is 8 bits. In decimal notation, a byte is a number in the range 0 to 255. You've probably heard that computers only understand 1's and 0's. While that's kinda true, they generally communicate using bytes. In Rust, bytes are represented by the type `u8`.

Bytes are fun because their meaning is usually open to interpretation. We've already seen that individual bytes can be interpreted as 8 individual bits or as a number from 0 to 255. They can also be interpreted in groups. A 32-bit integer is 4 bytes. Rust even gives you a function to make a `u32` from 4 bytes using `u32::from_be_bytes([1, 2, 3, 4])`. A Rust `String` is just a `Vec<u8>` whose bytes have been validated as UTF-8. Guess what the "8" in UTF-8 stands for.

Now let's make some chunks.


## Assignment
Using the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html), implement chunk types. Don't worry about implementing any specific chunk types. You only need to store a valid PNG chunk type. You don't have to implement the full chunk object either. We'll do that in the next chapter. This is just the 4 byte chunk type described in section `3.3` of the link above.

You need to provide methods that return the chunk type in bytes, check the validity of the entire chunk type, and check the special meaning of capitalization for each of the four bytes. Method signatures are provided below.

You will also need to implement a few standard library traits.


## Requirements
1. Copy the unit tests at the bottom of this page and paste them at the bottom of your `chunk_type.rs` file.
2. Write a `ChunkType` struct with your implementation of PNG chunk types.
3. Implement `TryFrom<[u8; 4]>` for your `ChunkType`.
4. Implement `FromStr` for your `ChunkType`.
5. Implement `Display` for your `ChunkType`.
6. Required methods:
   1. `fn bytes(&self) -> &[u8; 4]`
   2. `fn is_valid(&self) -> bool`
   3. `fn is_critical(&self) -> bool`
   4. `fn is_public(&self) -> bool`
   5. `fn is_reserved_bit_valid(&self) -> bool`
   6. `fn is_safe_to_copy(&self) -> bool` 
7. Pass all of the unit tests.


## Unit Tests

```rust
{{#include tests/chunk_type_tests.rs}}
```

## Resources
* [PNG Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)
* [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
* [std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
* [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)
* [std::str::from_utf8](https://doc.rust-lang.org/std/str/fn.from_utf8.html)
* [str::as_bytes](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes)
* [str::is_ascii](https://doc.rust-lang.org/std/primitive.str.html#method.is_ascii)
* [u8::is_acsii_lowercase](https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_lowercase)
* [u8::is_acsii_uppercase](https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_uppercase)
