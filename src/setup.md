# Setup

Create a new binary project using `cargo new --bin pngme`. My implementation is called `pngme` because I think I'm clever. Feel free to name yours whatever you want.

Copy the following code into your `main.rs` file.

```rust
{{#include stubs/main_full.rs}}
```

Your project will need to match this module structure in order to for the unit tests to pass. The easiest way to set this up is to make a separate file for each module. For example, create a file called `chunk.rs` in your `src` folder. The code in this file will be your `chunk` module. See the [module chapters](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html.
) in the book for more info.

I've provided `Error` and `Result` type aliases that will make it easy to use the `?` operator in your code. My own implementation uses the [anyhow](https://github.com/dtolnay/anyhow) crate for error handling. It's got some nice macros like `bail!` that make it easy to return errors in the middle of a function. If you're reading this in the future, we've all probably moved on from `anyhow` to some other fancy error handling crate. You do you.

