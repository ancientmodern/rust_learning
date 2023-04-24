fn main() {
    // The push_str method takes a string slice because
    // we don’t necessarily want to take ownership of the parameter.
    let mut s1 = String::from("foo");
    let s2 = "ba";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    s1.push('r'); // push a single character

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // note s1 has been moved here and can no longer be used, while s2 is still valid
    // 1. add takes ownership of s1,
    // 2. it appends a copy of the contents of s2 to s1,
    // 3. and then it returns back ownership of s1.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! macro uses references so that this call doesn’t take ownership of any of its parameters.
    let s = format!("{}-{}-{}", s1, s2, s3);

    // Rust strings do not support indexing, because of UTF-8 encoding,
    // and indexing into a string is not a constant time operation.
    // let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1];
    // panic: byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`

    // Iterating over Strings for Unicode Scalar Values
    for c in "Зд".chars() {
        println!("{}", c);
    }

    // Iterating over Strings for Bytes
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
