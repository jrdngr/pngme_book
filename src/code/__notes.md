Goals:
1. Provide an assignment
   1. Create a CLI app that works with secret messages hidden in PNG files
      1. Encode a message
      2. Decode a message
      3. Remove a message
      4. Print a list of chunks that can be searched for messages
2. Provide unit tests that verify the required features
   1. Give type names for anything that has to be unit tested
   2. Provide additional unit tests commented out that serve as hints for how to accomplish the assignment
3. Provide links to useful resources applicable to each chapter
   1. Try to offer at least two links for each thing in case one of them stops being valid in the future
   2. Links shared between chapters should be provided in each chapter
      1. The goal is for the reader to use resources
      2. I want them to focus on writing code and keeping momentum
   3. Suggested search phrases should also be provided 
4. Provide some small amount of discussion before the unit tests
   1. Not sure what this will be like yet but I'll have a better idea once the unit tests are written
5. Do not provide the completed code
   1. This is an intermediate tutorial
   2. If you read the book and you have some experience writing code, you can do this yourself
   3. I'll send my code to anyone who sends me their code
6. Minimize friction
   1. It should be easy to find the information you need. Let reading the info be the challenge
   2. Unit tests should be plentiful enough that they serve as real useful hints
7. Add unit tests for different languages
   1. This would be fun for other people to do
   2. It's basically an easy open source contribution
   3. Maybe Takumi will do Kotlin
   4. This is also a good learning opportunity
   5. Provide a list of required tests
   6. They can add any extra they want
   7. They can decide which unit tests start commented out
      1. Or I can do that if they don't care


# Outline

1. You want to start withsomething engaging and easy that doesn't require any extra reading
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
   2. Just make the suggestion to use expect
   3. This is an important shortcut for early development, but make sure you keep Result in mind
7. Using your own code to make something
   1. The best way to judge whether or not your code is good is to use it to make something
   2. If there's something unpleasant about your library, fix it. Make it more pleasant
   3. You'll probably be inspired to add your own features. Stop what you think you should be doing and implement those features
   4. This is definitely the last chapter. End stong.
10. Finding and using crates (or rolling your own)
    1.  clap, read args
        1.  Point to std docs for manual
    2.  anyhow, Box<dyn Error>, Custom error
        1.  Push anyhow. They can learn the downsides on their own.
    3.  crc, read another spec if you're into that sort of thing
11. Maybe. I haven't decided yet
    1.  There should probably be a real enum somewhere
        1.  These are just fun
        2.  `if let` is super useful to know and easy to forget
    2.  Implement an iterator
        1.  This is kinda irritating and frequently unnecessary
        2.  I can still mention it as an option
        3.  It's a good thing to learn how to implement because it's a bit more involved.
