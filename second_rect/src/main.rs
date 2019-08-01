struct Rectangle {
    width : i32, 
    height : i32
}

impl Rectangle{
    fn area(&self) -> i32{
        self.width * self.height
    }

    fn can_contain(&self, other : &Rectangle) -> bool{
        self.width > other.width && self.height > other.width
    }

    fn create_square(size : i32) -> Rectangle{
        Rectangle {width : size, height : size}
    }
}
fn main(){
    let rect1 = Rectangle {width: 10, height : 10};
    let _rect2 = Rectangle {width: 5, height: 5};

    println!("{}", rect1.area());
    println!("{}", rect1.can_contain(&rect1));

    let square1 = Rectangle::create_square(4);

    println!("{}", square1.area())
}