# Chapter 5 Hints

## Working with files

There are some [nice functions](https://doc.rust-lang.org/std/fs/index.html#functions) in the standard library for reading and writing files. [std::fs::read](https://doc.rust-lang.org/std/fs/fn.read.html) and [std::fs::write](https://doc.rust-lang.org/std/fs/fn.write.html) in particular.

If you're writing a function that reads from a file, there's a nice way to accept the file's path as a parameter using the [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html) trait. Your function signature will look something like `fn from_file<P: AsRef<Path>>(path: P)`.

## Working with strings and bytes

See the [Chapter 2 Hints](./chapter_2_hints.md)


## Stubs

```rust
{{#include ../stubs/commands_stubs.rs}}
```
