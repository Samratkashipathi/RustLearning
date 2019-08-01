#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Assosiaction function
    fn return_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }

    // fn increase_height(mut self) {
    //     self.height += 10;
    // }
}

fn main() {
    let mut rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("Area of rectangle {}", rect.area());

    println!("Is rectangle {:#?} square? -> {}", rect, rect.is_square());

    rect = Rectangle {
        width: 20,
        height: 20,
    };
    println!("Area of rectangle {}", rect.area());

    println!("Is rectangle {:#?} square? -> {}", rect, rect.is_square());

    // rect.increase_height();

    // println!("Rectangle after incereasing height: {:#?}", rect);

    let square = Rectangle::return_square(10); // Accesing Assosation functions using '::'
    println!("\nSquare : {:#?}", square);
}
