# Chapter 6 - Next Step

If you're reading this, hopefully you just got done posting a bunch of inappropriate messages to your company's Slack hidden inside innocent cat pictures. I know that's the first thing I did.

So what's next?


## If you want to keep working on this program
 1. Split your code up into a library and binary
    * [Cargo Project Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
 2. Add support for URL inputs and downloading images from the internet
 3. Add support for other file types
    * [Chunk-based file formats](https://en.wikipedia.org/wiki/File_format#Chunk-based_formats)
 4. Add an option to encrypt or obfuscate your hidden messages
 5. Figure out a way to automatically detect potential messages hidden in a PNG file



## If you like playing with bytes
1. Implement a [CHIP-8 Interpreter](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
    * This is the best next project IMO. If you're self-taught like me, you probably have a lot of little computer science knowledge gaps that you don't know about. You're basically making a CPU simulator here. You'll get exposure to a lot of things that most people don't normally encounter during their normal programming adventures.
2. [cryptopals](https://cryptopals.com/)
    * Sort of like Project Euler for cryptography. No prerequisite knowledge required.


## If you like implementing a spec
 1. Implement a [CHIP-8 Interpreter](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
    * For real, go do this.
 2. Implement PNG image decoding
    * Good luck
 3. [Advent of Code 2019](https://adventofcode.com/2019) has a really fun sequence of challenges that have you build your own little code interpreter.
    * This is more of a guided experience than diving into CHIP-8 if that's your thing 