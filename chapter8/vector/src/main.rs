fn main() {
    // Creating a new vector
    let mut v = vec![1, 2, 3, 4, 5];

    // Updating a vector
    v.push(6);
    v.push(7);

    // Reading elements of a vector
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(10) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let first = &v[0];

    // Can't mutate a vector while we have an immutable reference to an item
    // v.push(6);

    println!("The first element is: {}", first);

    // Iterating over the values in a vector
    for i in &v {
        println!("{}", i);
    }

    // Iterating over the values in a vector while mutating each value
    // let idx = 0;
    for i in &mut v {
        *i += 50;
        // Error: cannot borrow `v` as immutable because it is also borrowed as mutable
        // println!("{}", v[idx]);
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
