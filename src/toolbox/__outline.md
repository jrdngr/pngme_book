# Outline

1. What to start with
   1. Something engaging and easy 
   2. Possibly that doesn't require any extra reading
   3. Dive right in. No hello worlds
   4. Maybe write a tool that makes working with chunks easier
      1. Byte reading abstraction
      2. Chained iterators (reimplement built-in .chain())
      3. .read_exact() is my favorite part of this project
         1. We can probably make an iterator for this
         2. Give it a byte slice, get back 4 bytes at a time
2. Command line arguments
   1. I think this is a better second task since it will require some reading about parsing args
   2. Clap is a good library to start with now that it has StructOpt built in
   3. Provide the four commands and maybe the required parameters
3. PNG spec
   1. This might be too much reading to start with
   2. Maybe reading to start with isn't bad
   3. This is also good because it makes you feel smart to implement a spec for something you're familiar with
   4. Be really clear about where to look
4. Chunks
   1. This might be the most interesting part of the spec to implement
   2. It's simple but it feels complicated
   3. It's a good intro to working directly with bytes
   4. It's got easy unit tests
5. Common traits
   1. This should just happen throughout 
   2. Implementing Display is the most immediately helpful
   3. We'll give lots of hints to encourage people to implement From and TryFrom
      1. Maybe have a unit test that requires it later 
6. Error handling
   1. I definitely want to show the unwrap/expect -> Result process
   2. Just make the suggestion  to use expect
   3. This is an important shortcut for early development, but make sure you keep Result in mind
7. Using your own code to make something
   1. The best way to judge whether or not your code is good is to use it to make something
   2. If there's something unpleasant about your library, fix it. Make it more pleasant
   3. You'll probably be inspired to add your own features. Stop what you think you should be doing and implement those features
   4. This is definitely the last chapter. End stong.
8.  Finding and using crates (or rolling your own)
    1.  clap, read args
        1.  Point to std docs for manual
    2.  anyhow, Box<dyn Error>, Custom error
        1.  Push anyhow. They can learn the downsides on their own.
    3.  crc, read another spec if you're into that sort of thing
9.  Maybe. I haven't decided yet
    1.  There should probably be a real enum somewhere
        1.  These are just fun
        2.  `if let` is super useful to know and easy to forget
10. Implement an iterator
    1.  This is kinda irritating and frequently unnecessary
    2.  I can still mention it as an option
    3.  It's a good thing to learn how to implement because it's a bit more involved.
11. Macros
    1.  I think macros are beyond the scope of this book
    2.  It might be cool to have macro unit tests that look at the structure of their code
    3.  It would be an example of a macro that the reader can dig into if they want

    4.  Macro unit tests should probably start commented out
12. What to do next
    1. If you liked implementing a spec
        1. Implement PNG image decoding
        2. CHIP-8
        3. Add support for other file types
    2. If you liked making a cli tool
        1. Add support for urls/downloading
    3. If you like playing with bytes
        1. Crypto pals
        2. CHIP-8
