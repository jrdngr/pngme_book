# Chapter 3 Hints

## Reading files

You can easily read a file's bytes using [std::fs::read](https://doc.rust-lang.org/std/fs/fn.read.html). This will return the bytes in a `Vec<u8>` if it was successful.

If you're writing a function that reads from a file, there's a nice way to accept the file's path as a parameter using the [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html) trait. Your function signature will look something like `fn from_file<P: AsRef<Path>>(path: P)`.


## Working with bytes and iterators

Check out the [Chapter 2 Hints](./chapter_2_hints.md) for some tips about working with bytes and iterators.


## Stubs

```rust
{{#include ../stubs/png_stubs.rs}}
```
