# Chapter 4

If you made it this far, the hard part is over. Congrats!

We're going to take a little detour right now and get our command line arguments set up. Having arguments set up will make it really easy to work on the last part of this project.

You can parse command line arguments manually, but I highly recommend that you use a crate for this. I consider [clap](https://github.com/clap-rs/clap) to be one of those essential crates. It does all of the work to make a nice command line interface for you. All you have to do is get your data out. 

At the time of this writing, `clap` is integrating the derive macros from another fantastic crate called [structopt](https://github.com/TeXitoi/structopt). These macros make it trivial to map arguments to Rust types. You can check out version `3.0.0-beta.1` of `clap` to use these macros, or you can use `structopt`.

I put my argument types in a module called `args`. There aren't any more unit tests so you can put yours wherever your want.

When you're done, you should be able to run your program with commands similar to this:

`pngme encode ./dice.png ruSt "This is a secret message!`

`pngme decode ./dice.png ruSt`

`pngme remove ./dice.png ruSt`

`pngme print ./dice.png`



## Assignment
Add command line argument support. 


## Requirements
You should have four subcommands each with their own set of parameters.

### Encode
* File path
* Chunk type
* Message
* Output file (optional)

### Decode
* File path
* Chunk type

### Remove
* File path
* Chunk type

### Print
* File path


## Resources
* [std::path::PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
* [clap](https://github.com/clap-rs/clap)
* [structopt](https://github.com/TeXitoi/structopt)
* [std::env::args](https://doc.rust-lang.org/stable/std/env/fn.args.html)
* [Accepting Command Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
