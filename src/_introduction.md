# Introduction

"I just finished reading The Book. What should I do next?"

This question comes up weekly in online Rust discussions. The answers to this question are always "Make something!", sometimes followed by a list a cool projects. This is a great answer for some people, but others might be looking for a little more direction. 

This tutorial is intended to fill the gap between heavily directed beginner tutorials and working on your own projects. The primary goal here is to get you writing code. The secondary goal is to get you reading documentation. I'm assuming that you already know how to search the internet for information, so I'm going to give you links to all of the docs that you'll need to complete each chapter.

Unlike many of the other tutorials you might have worked through, I will not be providing any completed code. If you need extra help, there are hints for each chapter along with code stubs that can you get started. The first three chapters come with comprehensive unit tests that will ensure your code has the required features to move on. You'll use that code to complete the remaining chapters.

If you haven't read [The Rust Programming Language](https://doc.rust-lang.org/book/) yet, I highly encourage you to do so before attempting this tutorial. I will not be covering the usage of most language features.


## What are we making?

We're making a command line program that lets you hide secret messages in PNG files. Your program will have four commands:

1. Encode a message into a PNG file
2. Decode a message stored in a PNG file
3. Remove a message from a PNG file
4. Print a list of PNG chunks that can be searched for messages

If that sounds scary and beyond your ability then this tutorial is _definitely_ for you. If you know how to program and you know your Rust basics, you can totally do this.

It's important to remember that that this is __your__ project. You can implement it however you want. If you have ideas for cool features, go ahead and write them!

Heck, you might not even need this tutorial. If you want to tackle this whole project yourself, [this page](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) is all you really need. Go to town.
