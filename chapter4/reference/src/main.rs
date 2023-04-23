// 1. At any given time, you can have either one mutable reference
//    or any number of immutable references.
// 2. References must always be valid.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Important: only one mutable reference to a particular piece of data in a particular scope
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    let mut s = String::from("hello");

    change(&mut s);

    // We can use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    // example of the scope of a reference:
    // starts from where it is introduced and
    // continues through the last time that reference is used
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// pass a reference to a String, the original String is not moved
// references are immutable by default: borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
    // s.push_str(", world"); error: cannot borrow `*s` as mutable, as it is behind a `&` reference
}

// mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Example of dangling reference

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
}
// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}