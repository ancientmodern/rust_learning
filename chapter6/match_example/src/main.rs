enum Location {
    Point(i32),
    Range(i32, i32),
}

#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn main() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,
        Location::Range(_, n) => n,
        Location::Range(0, _) => 0,
        _ => -2,
    };
    println!("{n}");

    // Does not compile: The match arm Either::Right(s) moves the field s, so x cannot be used in the println.

    // let x = Either::Right(String::from("Hello world"));
    // let value = match x {
    //     Either::Left(n) => n,
    //     Either::Right(s) => s.len(),
    // };
    // println!("{x:?} {value}");

    let x = Either::Right(String::from("Hello world"));
    let value = match &x {
        Either::Left(n) => *n,
        Either::Right(s) => s.len(),
    };
    println!("{:?} {}", x, value);
}

fn decr_twice_v1(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2),
    }
}

fn decr_twice_v2(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}
