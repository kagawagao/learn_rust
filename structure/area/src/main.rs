#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of rectangle is {} square pixels.",
        area(width, height)
    );

    println!(
        "The area of rectangle is {} square pixels.",
        area_tuple((width, height))
    );

    let rect = Rectangle {
        width: dbg!(width),
        height,
    };

    println!(
        "The area of rectangle is {} square pixels.",
        area_struct(&rect)
    );

    println!("rect is {:#?}", rect);

    dbg!(&rect);

    println!("The area of rectangle is {} square pixels.", rect.area());

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    let square = Rectangle::square(30);

    println!("square is {:#?}", square);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
