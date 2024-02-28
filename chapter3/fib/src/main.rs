use std::io;

fn main() {
    loop {
        println!("Please input the index of the Fibonacci number:");

        let mut index = String::new();

        io::stdin().read_line(&mut index).expect("Failed to read line");

        let index: u64 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number, try again!");
                continue;
            },
        };

        println!("Result:\n{}", fib(index));
        break; // Break the loop after a successful operation
    }
}

fn fib(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}