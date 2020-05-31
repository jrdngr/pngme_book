# Chapter 1

Okay here we go!




## Assignment
Using the [PNG spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html), implement chunk types. This is one of the four pieces of data included in PNG chunk. Don't worry about implementing any of the chunk types specified in the spec. You only need to store a valid PNG chunk type.

## Requirements
1. Copy the code below into your editor.
2. Fill in the `ChunkType` struct with your implementation.
3. Implement `TryFrom<[u8; 4]>` for your `ChunkType`.
4. Implement `FromStr` for your `ChunkType`.
5. Implement `Display` for your `ChunkType`.
6. Required methods:
   1. `fn bytes(&self) -> &[u8]`
   2. `fn is_valid(&self) -> bool`
   3. `fn is_critical(&self) -> bool`
   4. `fn is_public(&self) -> bool`
   5. `fn is_reserved_bit_valid(&self) -> bool`
   6. `fn is_safe_to_copy(&self) -> bool` 
7. Pass all of the unit tests.

## Suggestions
* When I'm working directly with bytes, it can be helpful to manually align my code. If you're using `rustfmt`, you can write `#[rustfmt::skip]` above a function to keep your formatting intact.

## Resources
* [PNG Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)
* [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
* [std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
* [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)


```rust
{{#include tests/chunk_type_tests.rs}}
```
