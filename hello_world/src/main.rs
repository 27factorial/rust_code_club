// Hey! I'm Gabe. You might know me from CS 142 or 143, but I'm a student here
// at TCC studying computer science, mainly game development and systems
// programming. I wanted to do a presentation on Rust, since it has been my main
// language of choice after finishing the CS courses here.

// So, what is Rust? Rust is a systems programming language. It's mostly intended
// for lower-level applications, such as operating systems, drivers, networking
// code, and other such things. It has the benefit of having no garbage collector,
// while also being entirely memory safe. This means no "stop-the-world" pauses
// for the garbage collector to run, and you're safe from many bugs that other
// systems programming languages would allow you to write. The language also
// takes inspiration from the likes of functional programming languages such as
// Haskell, with patterns, pattern matching, iterators, higher-order functions,
// and lambda functions (also called closures). All of this leads to a language
// that is very nice to reason about, while also allowing you the access to
// lower level operations when you need them.

// The main goals of Rust are memory safety and speed of code. Memory safety is
// achieved with the ownership system, and Rust code is incredibly fast due to
// the compiler making _very_ aggressive optimizations. While a language itself
// can not be "fast", its implementation can do a lot to determine how quickly
// code runs.

// While I'm doing this presentation, keep in mind that Rust is a very new
// language. It was only first created in 2010, and is still constantly changing
// today. What I talk about may be subject to change over the next few years,
// but many of the concepts are here to stay in one form or another. Rust is
// also "split" one could say. There is Safe Rust and Unsafe Rust. The former
// is what most applications use, but operations requiring unsafe concepts such
// as raw pointers, low level memory operations, or sharing between threads
// require unsafe code, which relaxes Rust's restrictions a bit. What I'll be 
// going over is the safe part of Rust, which is the main component. To learn about
// Unsafe Rust, you need to know about safe programming first, and there is an
// entire book called the "Rustonomicon" that deals with compiler internals and
// unsafe code.

// Also keep in mind that Rust is definitely not object-oriented. This means that
// many of the things that you have learned about Java may not be applicable here,
// such as inheritance. Those who are used to languages like C++, Java, C#, or
// Python may have a bit of a harder time understanding why things are the way
// that they are in the language. While there are some object oriented things, 
// Rust is more like a mix between C and Haskell.

// So, without any more warnings or further ado, let's begin!

// Let's start with the most basic of programs. The one that everyone who starts
// their programming career starts with. The humble "hello world" program.

// Like Java, all Rust programs require a main function. This is where code
// execution starts. Unlike Java, the main method does not take any parameters,
// and rather arguments are dealt with differently.

// Functions must have 4 things.
// 1. The fn keyword.
// 2. The name of the function. Unlike Java, this is usually snake_case.
// 3. The function's parameters. How parameters are written will be dealt with
//    in a moment.
// 4. The body of the function.
fn main() {
    // This is a special concept called a macro. Rust macros are not functions,
    // but they can take arguments, and expand to some other code based on
    // those arguments at compile time. This is the println macro, which is used
    // to print things to the screen. It takes a string literal with optional
    // formatting characters. The real difference between macros and functions
    // does not matter right now, but macros always end their name with a !.
    println!("Hello, world!");
}
