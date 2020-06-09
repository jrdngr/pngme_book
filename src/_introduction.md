# Introduction

"I just finished reading [The Book](https://doc.rust-lang.org/book/). What should I do next?"

This question comes up weekly in online Rust discussions. The answers to this question are always "Make something!", sometimes followed by a list a cool projects. This is a great answer for some people, but others might be looking for a little more direction. 

This guide is intended to fill the gap between heavily directed beginner tutorials and working on your own projects. The primary goal here is to get you writing code. The secondary goal is to get you reading documentation.

If you haven't read [The Rust Programming Language](https://doc.rust-lang.org/book/) yet, I highly encourage you to do so before attempting this project. This guide does not cover any language features.


## What are we making?

We're making a command line program that lets you hide secret messages in PNG files. Your program will have four commands:

1. Encode a message into a PNG file
2. Decode a message stored in a PNG file
3. Remove a message from a PNG file
4. Print a list of PNG chunks that can be searched for messages

If that sounds scary and beyond your ability then this guide is _definitely_ for you. If you know how to write code and you know your Rust basics, you can totally do this. We're not going to implement any sort of image decoding. The part of the PNG spec we're tackling is surprisingly simple.


## How this works

Unlike many of the other tutorials you may have worked through, I will not be providing any completed code. If you need extra help, there are hints for each chapter along with code stubs that can you get started. The first three chapters come with comprehensive unit tests that will ensure your code has the features it needs. You'll use your unit tested code to complete the remaining chapters.

This is supposed to be _your_ project, not mine. You can use as much or as little of this material as you want. You know your learning style better than I do, so do what works best for you. You don't need to follow my suggestions or click my links or use my unit tests if you don't want to, but you can always come back to them if you get stuck. 

I'll give you links to documentation for everything I used in my own implementation of this project, but it's up to you to figure out how to use what you've read. While you're reading those docs, dig around a little bit and see what else is available. Rust's standard library types provide a ton of useful functionality and you might like something else more than what I linked.

If you want a more regimented plan, here it is.

For each chapter:
1. Read the intro
2. Read the requirements 
3. Copy the unit tests into your project
4. Read the relevant section of the PNG spec
5. Write your code and try to pass the tests
6. If you get stuck, check out the provided list of resources
7. If you're still stuck, check out the hint page for that chapter
8. If you can't figure it out, ask the internet. The Rust community is very helpful.

Each chapter is already laid out in this order so it should be easy to follow.

If you really really super duper want to see my code, you're gonna have to [send me your code](https://twitter.com/picklenrd) first.
