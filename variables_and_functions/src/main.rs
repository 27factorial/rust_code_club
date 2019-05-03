// Like Java, the main function is required to be in every binary application.
// However, unlike Java, There's no String[] args in main, and arguments are
// actually handled through a special iterator in the `env` crate.
fn main() {
    // In my last example, I annotated types for every declaration. This is
    // actually non-idiomatic.
    // Notice, there's no type annotation! This is due to Rust's type inference,
    // which will allow you to elide type annotations in many cases. Types can
    // be determined at compile time using something called Hindley-Milner
    // Type Inference.
    let x = 5;
    let y = 6;

    // A function call with no parameters. Notice the name of all functions. They
    // are always in snake_case, and not camelCase like in Java. This is just
    // convention.
    say_hello();

    // Yet again, no type annotations.
    let name = String::from("Gabe");

    // Passing a parameter to a function. This function returns nothing, but
    // just prints to stdout.
    say_hello_to_name(name);

    // There's something odd about `name`, though. Notice that if you attempt
    // to call say_hello_to_name again, it will throw a compiler error. This
    // is because of a concept known as Ownership, and it is core to Rust's
    // design. Ownership will be talked about more in a bit.
    // use of moved value: `name`
    // value used here after move. rustc(E0382)
    // say_hello_to_name(name);

    // Calling a function with 2 parameters and
    // printing its result.
    let result = add(x, y);

    println!("{}", result);

    // Interestingly, `x` and `y` are still perfectly valid after being used in 
    // add. This is because all primitive types (besides str) implement a trait
    // called Copy, which means that their values are simply _copied_ instead
    // of moved when they're used. This is because copying an integer is a very
    // inexpensive operation, versus copying a whole Vec or String into a function.
    println!("x = {} and y = {}", x, y);

    // We talked about type inference earlier, but there are some cases where
    // the compiler can not infer the type that you want. This is when you
    // must manually annotate the type. In this case, Vec is generic over type T,
    // which means that T can be any type, and the compiler can't figure out which
    // Vec you really want.
    // type annotations needed
    // cannot infer type for `T` rustc(E0282)
    // let broken_vec = Vec::new();

    // In this case, we can use the <name>: <type> syntax talked about earlier.
    // The underscore is just so that the compiler does not complain, since we
    // aren't using the variable.
    let _working_vec: Vec<i32> = Vec::new();

    // However, types may be rarely ever needed, because if the Vec is used in
    // a context where the type can be inferred, then the compiler doesn't
    // require type annotations. For example: A function that takes a Vec and
    // an i32 and adds that i32 to the Vec will make the Vec<T> become Vec<i32>
    // if the compiler didn't already know its type before calling the function.

    // For those of you who are enrolled or have completed CS 143, you'll know
    // that Java only supports making _Objects_ generic. To have a generic
    // primitive, you must wrap it in an Object, such as Integer, or Double.
    // This is not the case in Rust. Any type, primitive or not, can be used
    // with generics.
    
    // If you feel like doing a bit of research, Rust handles generics in a way
    // called monomorphization. Because of this, there is no performance penalty
    // versus hand-writing each and every version of a function or type.
}

fn say_hello() {
    println!("Hello!")
}

// A function with a parameter. Parameters are written in the form <name>: <type>.
// Multiple arguments are seperated with commas. This function doesn't seem to 
// return anything, so there's no need to denote a return type.
fn say_hello_to_name(name: String) {
    // {} is a type of format string. It will read the first arguments after
    // the comma, and apply it to the first brace pair. It effectively replaces
    // the braces in the final version.
    println!("Hello, {}!", name);
}

// A function's return type is shown with the `->` operator.
fn add(first: i32, second: i32) -> i32 {
    // If an expression is the last expression in the function, the return 
    // keyword and semicolon can be omitted, and that expression will
    // implicitly be returned.
    first + second
}

// Let's talk for a moment about no return type. Every function in Rust must
// return _something_, and can not just return nothing. This is in contrast
// to the `void` return type in Java, where that literally denotes nothing to
// return. To get around this, Rust has a type called Unit, which is a type
// that represents nothing to be returned. This is so that returning nothing
// can be consistent with the type system, as a variable can not have type void
// in Java, but a variable in Rust can have the type and value of Unit. Unit
// is written as (), which is an empty tuple.

// Functions that have no arrow are presumed to return the unit type, so the
// say_hello_to_name function prototype could be written like so:

// fn say_hello_to_name(name: String) -> () { ... }

// However, this is unidiomatic, because it makes no difference, and can be
// confusing to people reading your code.

