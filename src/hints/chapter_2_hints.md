# Chapter 2 Hints

## Getting bytes from a string
You can turn a `&str` into a `Vec<u8>` using [str::bytes](https://doc.rust-lang.org/beta/std/primitive.str.html#method.bytes). 

`let bytes: Vec<u8> = your_str.bytes().collect();`
   
## Getting a string from bytes
You can turn a `Vec<u8>` into a `String` using [String::from_utf8](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8). 


## Working with iterators
You can chain iterators together using [Iterator::chain](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain).

You might run into trouble getting the types of your iterators to match. You can use [Iterator::copied](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied) or 
[Iterator::cloned](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned) to ensure that your iterator is working with values rather than references.


```rust
let result = first_collection
    .iter()
    .cloned()
    .chain(second_collection.iter().cloned())
    .chain(third_collection.iter().cloned())
    .collect();
```


## Reading bytes

You can turn a `[u8; 4]` into a `u32` with [u32::from_be_bytes](https://doc.rust-lang.org/std/primitive.u32.html#method.from_be_bytes). The `be` stands for *big endian*.

You can read a fixed number of bytes from a reader using 
[Read::read_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact). Chunks have several different 4-byte sequences.

```rust
let mut reader = BufReader::new(bytes);
let mut buffer: [u8; 4] = [0, 0, 0, 0];

reader.read_exact(&mut buffer)?;
let data_length = u32::from_be_bytes(buffer);
```


## Stubs

```rust
{{#include ../stubs/chunk_stubs.rs}}
```
