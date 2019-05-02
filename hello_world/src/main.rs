
// Like Java, all rust programs require a main function. This is where code
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
