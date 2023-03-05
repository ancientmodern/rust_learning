use std::io;

fn main() {
    // type must be annotated when there is no way to infer it
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // CHAR
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // TUPLE
    let x: (i32, f64) = (500, 6.4);
    let (a, b) = x;

    let first = x.0;
    let second = x.1;

    println!("a: {}, b: {}", a, b);
    println!("first: {}, second: {}", first, second);

    // ARRAY
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // this will panic if the index is out of bounds
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
