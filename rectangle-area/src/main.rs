fn main() {
    let a_rect = Rectangle {
        width: 2,
        height: 10,
    };

    let b_rect = Rectangle {
        width: 1,
        height: 5,
    };

    println!("The area of {:?} is {}", a_rect, a_rect.area());

    match a_rect.can_hold(&b_rect) {
        true => println!("{:?} can hold ${:?}", a_rect, b_rect),
        false => println!("{:?} cannot hold {:?}", a_rect, b_rect),
    }

    match b_rect.can_hold(&a_rect) {
        true => println!("{:?} can hold ${:?}", b_rect, a_rect),
        false => println!("{:?} cannot hold {:?}", b_rect, a_rect),
    }

    let a_square = Rectangle::square(5);

    println!("a_square: {:?}", a_square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}
