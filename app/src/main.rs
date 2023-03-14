#![deny(clippy::all)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, react: &Rectangle) -> bool {
        self.area() == react.area()
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
//All functions defined with an impl block is called assocaited functions because
// they are asssociated with the type named after the imple

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let sqr = Rectangle::square(2);

    println!(
        "The area of the rectangle is {} square pixels. {} sqr {}",
        rect1.area(),
        rect1.can_hold(&rect2),
        sqr.area()
    );
}

//`{:?}` (or {:#?}
