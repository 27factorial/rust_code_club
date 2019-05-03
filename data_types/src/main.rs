fn main() {
    // Let's create a variable with the type i32, which is an int in Java.
    // Variables are given types in the following format: <name>: <type>.
    // the keyword `let` is used to create variables.
    let var: i32 = 42;

    // Now let's try a char! Remember that a char is any one valid Unicode scalar
    // value. For example, the ß character.
    let esstzet: char = 'ß';

    // Let's try to create a str. Your first instinct might be to create a str
    // by writing this:
    // let my_str: str = "Hello, World!";
    // But notice, the compiler gives you an error if you try this. Why is that?
    // In Rust, every value must have a known size at compile time. In Java, this
    // isn't a problem, since everything is a reference, and references have a
    // fixed size, everything has a known size at compile time. In Rust, strings
    // might be gotten from somewhere outside of the program, such as a file
    // on disk, and as such, can not have a size known at compile time.
    // Also notice the other error the compiler is giving you. It's saying that
    // the string literal has the type "&'static str". What the static part means
    // is not relevant at the moment, but the & means a reference to a string.
    // If we make a reference to a string, a reference is just a pointer, like in
    // Java, so it has a known size at compile time, therefore, we can make
    // our string into a &str, and it will work! This reference holds some information
    // about the string, including its location and size, and therefore, reading
    // from it will read exactly that many characters, and we still have a fixed
    // size to store the string in.
    let _my_working_str: &str = "Hello, World!";
    // Now let's try indexing a str! Remember that Rust will crash the program if
    // you index at an invalid location, since UTF-8 is not fixed size. Let's see
    // What Rust does if we try to take the first character of this str, which
    // is "hello" in Chinese.
    let hello_str: &str = "你好";
    // The below must be a &str, since Rust can not know the size of it at
    // compile time, so we reference the indexing operation. This indexing syntax 
    // is called a range, and it denotes starting at the 0th byte inclusive and 
    // continuing until the 1st byte exclusive.
    let _my_value: &str = &hello_str[0..1];
    // The program crashes! This is because the 你 character is larger than one
    // byte, so storing only part of it into a string would give incorrect results.
    // There are ways to fix this, namely changing the 1 to 3 (since the
    // character is three bytes long), but also by using the Result enum, which
    // will be talked about later.

    // Rust also has a few "compound" types, namely, the tuple type and the array.
    // A tuple is a compound type that allows each of its members to have a
    // different type. For example:
    let unsigned_and_signed: (u32, i32) = (1024, -2048);
    let str_and_char: (&str, char) = ("This is a string", 'a');
    // Tuples can be indexed by a period and then an index.
    let _unsigned: u32 = unsigned_and_signed.0;
    let _a_char: char = str_and_char.1;

    // Arrays in Rust are almost the same as in Java, namely that they can hold
    // a fixed number of items and that the items must be of the same type.
    // However, trying to index an array out of bounds will be a compiler error
    // by default. Since the array size is known at compile time, Rust will check
    // to make sure any access to the array isn't out of bounds. (Technically,
    // this can be disabled, but why would you?) The type signature of an array
    // is [<type>; <length>].
    let array: [i32; 5] = [16, 17, 5, 4, 2002];
    // Indexing into an array is the same as Java also.
    let first_item: i32 = array[0];
}
