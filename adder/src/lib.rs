#[derive(Debug)]
struct Rectangle {
    width : i32,
    height : i32
}

impl Rectangle {
    fn can_hold(&self, other : &Rectangle) -> bool{
        println!("Checking...");
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn positive() {
        let larger = Rectangle {
            width : 4,
            height : 4
        };

        let smaller = Rectangle {
            width : 2,
            height : 2
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn negative(){
        let larger = Rectangle {
            width : 4,
            height : 4
        };

        let smaller = Rectangle {
            width : 2,
            height : 2
        };

        assert_eq!(false, smaller.can_hold(&larger))
    }
}
