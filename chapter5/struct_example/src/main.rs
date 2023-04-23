#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // only borrow, do not need ownership
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle { // example of Associated Functions
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect_tuple = (30, 50);

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    let mut rect1 = Rectangle {
        width: 10,
        length: 40,
    };

    println!("{}", area_tuple(rect_tuple));
    println!("{}", area(&rect));

    println!("{}", rect.area());

    println!("{:#?}", rect); // pretty output, value not change

    println!("{}", rect.can_hold(&rect1));
    rect1.length = 100;
    println!("{}", rect.can_hold(&rect1));

    let s = Rectangle::square(10);
    println!("{:#?}", s);
}

fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}