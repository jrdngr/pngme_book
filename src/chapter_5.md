# Chapter 5


This is my favorite part.

We've finished our library code and we've done the busywork to get input from the user via command line arguments. The only thing left to do is glue it all together.

You're using your own code to write more code here, which means you'll be able to evaluate what you wrote before from the perspective of a user. If you think your library code is annoying to work with, now is the time to revisit it. I like to take a two pronged approach to my APIs. I have the flexible but potentially cumbersome base logic that does all of the heavy lifting, then I add a bunch of nice helper code to make the common use cases easier to work with.

I put my code for ths step in a module called `commands` because it's designed around the four subcommands that we support, but you can structure your code however you want.


## Assignment
Map your command line arguments to your PNG code and bring this project home!


## Resources
* [std::fs](https://doc.rust-lang.org/std/fs/index.html)
