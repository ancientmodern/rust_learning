fn main() {
    // If there is no `mut`, by default, the variable is immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // The difference between `let` and `const`:
    // 1. `const` must be annotated with type
    // 2. `const` can be declared in any scope, including global scope
    // 3. `const` can only be set to a constant expression, not the result of
    //     a function call or any other value that could only be computed at runtime
    // `const` is similar to `constexpr` in C++, but lack of many functionalities at this time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Example of shadowing
    let x = 5; // x == 5
    let x = x + 1; // This `x` shadows the above one, x == 6
    {
        let x = x * 2; // This `x` shadows the outer one, x == 12
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}"); // x == 6

    // Shadowing can be used to change the type of variable, as we did in the guessing game
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");
}
