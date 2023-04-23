// use std::collections::HashMap;
use std::collections::LinkedList as LL;
use std::collections::{self, HashMap};

pub mod point {
    #[derive(Debug)]
    pub struct Point(pub i32, i32);
    impl Point {
        pub fn origin() -> Self {
            Point(0, 0)
        }
    }
}

fn main() {
    let mut p = point::Point::origin();
    p.0 += 1;
    // p.1 += 1;
    println!("{p:?}");

    let mut map = HashMap::new();
    map.insert(1, 2);

    let linkedlist1: LL<i32> = LL::new();
    let linkedlist2: collections::LinkedList<u32> = LL::new();
}
