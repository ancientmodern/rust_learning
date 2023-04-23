#[derive(Debug)]
pub struct Asparagus {
    name: String,
    year: u32,
    alive: bool,
}

impl Asparagus {
    pub fn new(name: String, year: u32, alive: bool) -> Self {
        Asparagus { name, year, alive }
    }
}
