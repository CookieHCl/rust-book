#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Self is alias for Rectangle
    // &self is short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &mut self is short for self: &mut Self
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // self is short for self: Self
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // associated functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // syntax sugar of Rectangle::area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    dbg!(&rect1); // takes ownership, uses pretty Debug formatting {:#?}

    // associated functions
    let sq = Rectangle::square(3);

    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    println!("{}", rect.area());

    rect.set_width(1);

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);
}
