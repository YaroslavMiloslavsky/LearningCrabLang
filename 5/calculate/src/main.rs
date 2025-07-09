#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, PartialEq)]
struct Rect {
    width: u64,
    height: u64,
}

#[allow(unused)]
impl Rect {
    // Self is alias for the type impl block is for
    
    fn new(width: u64, height: u64) -> Self {
        Self {
            width,
            height
        }
    }

    fn square(size: u64) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    // self: &Self == &self
    // ref is used because else, self would be dropped after area method call
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn expand_area_by_x(&mut self, x: u64) {
        self.width += x;
        self.height += x;
    }
}


fn main() {
    let r = Rect::new(50, 50);
    let r1 = Rect::square(50);

    println!("{}", r.eq(&r1));
    println!("{}", r.eq(&r1));
}
