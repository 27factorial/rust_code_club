// I mentioned ownership in my last example, but now we can finally get into
// what it means to "own" something. A "quick" overview of ownership first,
// though. Brace yourself for a long lecture.

// Take a phone as an example. Imagine a scenario where you're handing someone
// your phone. To model this, you can say that they own it, and that they
// it is theirs now. They give it back to you after they're done, saying that
// you own it now. this cycle can repeat over and over again. What you want to
// do is not to keep switching owners forever, but rather, you want them to be
// able to borrow it, and then give it back without ever losing ownership of it.
// This is easier, because instead of transferring owners every time they need
// to use your phone, you can let them borrow it with the assumption that
// they will give it back.

// This same concept applies to variables and values in Rust. Taking ownership
// of something means that the owner of that value is _entirely_ responsible
// for the value. This means that all memory mangement of the value is done
// by the owner, and no one else. After taking ownership, you can not use the
// value again (unless ownership was given back at some point). This means that
// Rust is immune to errors such as double frees. With this ownership system,
// every value has a specified place to be freed. When the object that owns the
// value, whether that be a function. struct, or enum goes out of scope, the
// value is cleaned up! (We'll learn about structs and enums next.)

// The described behavior is the reason that Rust does not need a garbage
// collector, like Java. In Java, values are valid as long as they have something
// that can access them, such as a variable, or class instance. They do not have
// rules for when to make a value go out of scope. This results in the application
// having to periodically search for values that have become garbage, and clean
// them up. This does not happen in Rust, since we know the value will be dropped
// whenever the thing that owns it goes out of scope.

// However, as noted in the example before, transferring ownership isn't always
// the best way to model a problem. Sometimes, you just want to let something
// borrow a value and give it back once it is done. The main way borrowing is
// done is through references, which are very similar to references in Java (in
// fact, everything except primitives is a reference in Java!). References are
// something that "points" to a value's location in memory, meaning that the
// value can be accessed without ever changing owners of the value itself.
// There are two types of references in Rust
// &     => the immutable reference. This allows something to point to a value, but
//          not to mutate it or change it in any way. This is useful for just reading
//          values that you want to be able to use later instead of taking ownership
//          of them. This is similar to only allowing someone to use your phone to
//          make a call, then taking the phone back from them so that they can not
//          see your personal information.
// &mut  => the mutable reference. This is a reference that allows mutation of
//          the value it points to. This is similar to allowing the person to
//          change all the settings on your phone before giving it back to you.
//          which would be useful in certain scenarios.

// Both of these are useful in situations where you don't necessarily want to
// give up ownership of something (e.g. Using a value later), but just want
// whatever is referencing it to be able to see or change the value.

// Rust's ownership system places some restrictions on references, though.
// These restrictions make sure that all references are valid, and won't ever
// point to garbage data (looking at you, C and C++). To do this, you must
// adhere to these rules.
// 1. Have as many immutable references as you would like. This is allowed, since
//    The data can not unexpectedly change from beneath the reference, possibly
//    causing the reference to be invalid or have unexpected values.
// 2. One mutable reference. If you want to have a mutable reference, there can
//    only ever be that reference in scope. This is for two reasons.
//      a) having two mutable references is not allowed, since that can allow for race
//    conditions and unexpected behavior. If two mutable references are active
//    at the same time, they may not produce the desired behavior. For example,
//    The reference might move a value to another place in memory, and then
//    the other mutable reference tries to change that value, but it is pointing
//    to garbage data now, making this undefined behavior.
//      b) immutable references aren't allowed, either. This is because if you have
//    a mutable reference and an immutable one, the immutable one may have some
//    assumption of the referenced value, which can be changed by the mutable
//    reference.
//    Mutable references are like letting someone borrow your phone to fix it,
//    but having multiple people looking at it or trying to fix it at the same
//    time makes the job harder.

// Wow, that's a lot of information to take in. It's all worth the read though,
// because these rules and concepts make references and ownership entirely
// memory safe. References and never be invalid, and values can not be used
// after they are moved to another location. This prevents a number of issues
// such as data races, double frees, and other undefined behavior. This also
// means that the compiler will always be able to validate references for you,
// making sure that these bugs can never happen!

// One other thing to note: values in Rust are immutable by default. This does
// not mean that there is no mutability in Rust, but rather, it must be
// explicitly declared that you want to make the value mutable. Immutable
// references of mutable values are still allowed in cases where you want to
// change the value, but don't need to change it in certain contexts, such as
// just reading the value from a variable.

fn main() {
    // Using the `mut` keyword to denote that this value can be changed.
    let mut name = String::from("Gabe");

    // References are denoted with the & operator, which creates a reference to
    // a value. This is an immutable reference, since there is no `mut` keyword
    // with the ampersand.
    say_hello(&name);

    // We can still use name here! We can modify it since it was marked with the
    // `mut` keyword when we defined it.
    // Notice this notation, <var>.<method>, this is similar to the same thing
    // in Java, where you're calling a method on a type. This method takes a
    // mutable reference to the variable, so ownership is never transfered!
    name.push_str(" Love");
    // name = Gabe Love

    // Let's call that say_hello() function again.
    // There's no compiler error!
    say_hello(&name);

    // Let's pass the value to a function that takes a mutable reference.
    // We use &mut to denote that we're taking a mutable reference.
    append_a(&mut name);
    // name = Gabe Lovea

    say_hello(&name);

    // uncomment the next three lines and see what the compiler tells you.
    // Try turning one of the references into an immutable reference. You
    // will get a similar error! it only errors after you use one of the
    // references, because having multiple mutable references and not doing
    // anything with them is obviously not dangerous. If you have two mutable
    // references or a mutable reference and an immutable one and try to use
    // one of them, the compiler gives you an error.
    // let name_mut1 = &mut name;
    // let name_mut2 = &mut name;
    // say_hello(&name_mut1);

    // `name` is dropped here, right before the main function exits! Since main()
    // owns the value (after all, it was defined in main), it will be dropped
    // when main() is not using it any longer.
}

// Notice that we take a &str here, but we can still use the function with &String.
// This is due to something called Deref coercion, which allows some types to be
// used as references of another type.
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn append_a(name: &mut String) {
    // This is similar to the .push_str() function, except that it only pushes
    // one character to the string.
    name.push('a');
}
