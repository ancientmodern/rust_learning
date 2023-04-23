use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus::new(String::from("lhr"), 22, true);
    println!("I'm growing {:?}!", plant);
}
