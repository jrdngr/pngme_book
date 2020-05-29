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


# Topics
1. Creating command line arguments
2. Reading the PNG spec
3. Implementing a simplified PNG spec
4. Implementing PNG chunks
5. Common standard library traits
   1. From
   2. Display
6. Slightly less common standard library traits
   1. TryFrom
7. Error handling
8. Using your own code to make something 
   1. Putting it all together
9. Working with bytes
10. There should probably be a real enum somewhere
11. Implement an iterator
12. Finding and using crates (or rolling your own)
    1.  clap, read args
    2.  anyhow, Box<dyn Error>, Custom error
    3.  crc, read another spec if you're into that sort of thing

These don't need to have their own lessons
* Common traits
* Error handling
* Using your code to make something


# Outline

1. You want to start withsomething engaging and easy that doesn't require any extra reading
2. Command line arguments
   1. I think this is a better second task since it will require some reading about parsing args
3. PNG spec
   1. This might be too much reading to start with
   2. Maybe reading to start with isn't bad
   3. This is also good because it makes you feel smart to implement a spec for something you're familiar with
4. Chunks
   1. This might be the most interesting part of the spec to implement
   2. It's simple but it feels complicated
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
8. 